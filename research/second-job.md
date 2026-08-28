# The second job advancement, at level 30

Working notes behind `crates/world/src/secondjob.rs`. Labels are the project's: **[L]** read
off this client's listing, its WZ or a capture, **[D]** derived from two or more [L] facts,
**[I]** inferred - a policy nothing on this machine can confirm.

**Nothing here authenticates.** As everywhere in this project, the channel socket carries no
credentials. A second job advancement is granted on the say-so of whoever holds the socket.

> ## IT IS NOT WIRED
>
> `crates/world/src/session/` belongs to the coordinator and was not touched. **Nothing calls
> anything in `secondjob.rs`.** On screen this is currently identical to the feature not
> existing. §9 is the "wire it like this". `CLAUDE.md` has a section called *"Built is not
> wired"* because two subsystems sat in exactly this state for a day while `STATUS.md` called
> them done.

No Ghidra pass and no client run went into this. Every number is out of the client's own WZ,
via `gm-handbook/` and `target/release/wz-dump.exe`.

---

## 1. The table

| from | instructor (**advances**) | on map | town | examiner (**tests only**) | on map | choices |
|---|---|---|---|---|---|---|
| 100 Swordsman | **511** Dances with Balrog | 10004003 Warriors' Sanctuary | 10004000 Perion | **514** Warrior Job Instructor | 10004023 West Rocky Mountain IV | 110, 120, 130 |
| 200 Magician | **313** Grendel the Really Old | 10002003 Magic Library | 10002000 Ellinia | **319** Magician Job Instructor | 10002070 The Forest North of Ellinia | 210, 220, 230 |
| 300 Archer | **221** Athena Pierce | 10001051 Bowman Instructional School | 10001000 Henesys | **227** Bowman Job Instructor | 10001090 The Road to the Dungeon | 310, 320 |
| 400 Rogue | **411** Dark Lord | 10003003 Thieves' Hideout | 10003000 Kerning City | **424** Thief Job Instructor | 10003080 Construction Site North of Kerning City | 410, 420 |

All **[L]**. Instructor and examiner names from `String.wz/Npc.img/<template>/name`
(`gm-handbook/npcstrings.txt`); placements from `Map.wz`'s `life` nodes
(`gm-handbook/npcs.txt`); map names from `gm-handbook/maps.txt`. Each of the eight NPCs is
placed **exactly once** in the whole archive.

### The ten second jobs

| job | name | provenance of the name | `String.wz/Skill.img/<job>/bookName` **[L]** | book | grantable |
|---|---|---|---|---|---|
| 110 | Fighter | **[D]** leading word of the book name | `Fighter Techniques` | 10 | **8** |
| 120 | Page | **[D]** | `Page's Path` | 10 | **8** |
| 130 | Spearman | **[D]** | `Spearman Techniques` | 10 | **8** |
| 210 | Wizard (Fire/Poison) | **[I]** *not in this client at all* | `Fire & Poison Basics` | 7 | **6** |
| 220 | Wizard (Ice/Lightning) | **[I]** *not in this client at all* | `Ice & Lightning Basics` | 6 | 6 |
| 230 | Cleric | **[I]** *not in this client at all* | `Holy Magic Basics` | 6 | 6 |
| 310 | Hunter | **[D]** | `Hunter's Guide` | 8 | **6** |
| 320 | Crossbowman | **[D]** | `Crossbowman Guide` | 7 | **6** |
| 410 | Assassin | **[D]** | `Assassin Skills` | 6 | 6 |
| 420 | Bandit | **[D]** | `Bandit's Tricks` | 6 | 6 |

**66 grantable second-job skills** across the ten books, out of 72 rows. §7 is the difference.

The job ids are **[L]**: the `job` column of `gm-handbook/skills.txt` is derived by
`tools/dump_skills.py` from the image each skill was found in (`Skill_000.wz/<book>.img`), and
enumerating it gives exactly 25 books - `0`, the four first jobs, these ten, and ten third-job
books `111 121 131 211 221 231 311 321 411 421`. **There is no fifth branch and no Pirate.**
This was enumerated, not assumed - `CLAUDE.md`'s *enumerate before you filter*.

---

## 2. The advancement is at the FIRST-job instructor, not at the examiner

This is the fact the module is shaped around, and it is the one a from-memory implementation
gets wrong.

`crates/world/src/jobs.rs` already says the four `<Job> Job Instructor` NPCs are *"the second
advancement's examiners"*. Correct - and incomplete. **They examine and they advance nobody.**
The chain ends back at the first-job instructor.

`gm-handbook/questlines.txt`, quest `20003` (the fourth of the Warrior chain):

```text
20003  Check  1.npc          511
20003  QuestInfo  2   #b#p511##k in #b#m10004000##k welcomed me warmly,
                      and I gained a new job, obtaining even greater power!
```

And the examiner's own idle line, `String.wz/Npc.img/514/idle0`:

> *"Do you want to undergo your 2nd job advancement as a warrior? Then go see Dances with
> Balrog in Perion."*

All four examiners carry the same sentence naming their own branch's instructor. **[L]**

### Consequence for the wiring

`511`, `313`, `221` and `411` now give **two** advancements. A click on one of them has to
consult `jobs::advancement_for` for a beginner and `secondjob::advancement_for` for a
first-job character. That is why `secondjob::Advancement` has a `StillABeginner` arm that
produces **no refusal sentence** - it is a hand-off, not a no. A caller that printed something
there would refuse a level-10 beginner who is entitled to a first advancement.

---

## 3. The quest chain, and why it is data rather than a gate

Four quests per branch, all sixteen carrying `Check.0.lvmin = 30` and a single
`Check.0.job.0` of `100`/`200`/`300`/`400`. **[L]**

```text
  20000  The Warrior's Next Journey   turn in at 511   ->  20001
  20001  Finding the Instructor       turn in at 514   ->  20002   (letter 4031013 handed out)
  20002  Test of Qualification        turn in at 514   ->  20003   (30 x 4031017 wanted)
  20003  Proof of Qualification       turn in at 511   ->  --      (proof 4031018 handed out)
```

| branch | quests | letter | marble (x30) | proof |
|---|---|---|---|---|
| Warrior | 20000..20003 | 4031013 | 4031017 | 4031018 |
| Magician | 20100..20103 | 4031014 | 4031019 | 4031020 |
| Bowman | 20200..20203 | 4031015 | 4031021 | 4031022 |
| Thief | 20300..20303 | 4031016 | 4031023 | 4031024 |

Every quest pays `Act.1.exp = 3150`, so a finished chain is **12 600 experience points**.
All **[L]**, and all asserted in `secondjob::tests::the_quest_chains_are_what_this_client_ships`
against the generated dump rather than transcribed into a comment.

**`secondjob::REQUIRE_QUEST_CHAIN` is `false`, and that is [I].** Quest `20002` is gated on
`Check.0.startscript = q20002s` and hands out its Black Marbles inside a hidden field this
server does not implement. Enforcing the chain would make the second advancement untestable,
which is the one thing a feature in this repo may not be. The chain is carried as
`secondjob::QuestChain` so a caller that *does* have quest state can enforce it in one place.

### The `Check.0.job.0` is a single entry

Not a list. There is no `Check.0.job.1` on any of the sixteen. **[L]** So *"a Magician cannot
take the Warrior's second advancement"* is the client's own rule and not one this server
invented, and `Advancement::WrongBranch` is quoting it.

---

## 4. The prerequisites, and the one this file refuses to invent

**Level 30** - **[L]** from `Check.0.lvmin`, sixteen times. Also stated in the quest's own
words: *"Having reached level 30 and mastered the way of the Swordsman..."*.

**Already the matching first job** - **[L]** from `Check.0.job.0`.

**Nothing else.** `crates/world/src/jobs.rs`'s `STAT_MINIMUM` is **[I]**, is the owner's 35 from a
fan site, was questioned once and left at 35, and there is nothing in this client to check it
against. **`secondjob.rs` deliberately adds no second number of that kind**, and the negative
is measured rather than assumed:

Every `Check` key shape across all **322** quests was enumerated - 37 distinct shapes with
their counts. The census reproduces the one `jobs.rs` recorded, exactly:

```text
  1.item.N.id  330   1.item.N.count 329   1.npc  320   0.npc  320   0.lvmin  317
  0.quest.N.state 215  0.quest.N.id 215   1.item.N.order 135   1.order 110
  0.job.N  104   1.lvmin  89   1.mob.N.id  87   1.mob.N.count  87
  0.citizenshipTown 86   0.citizenshipGrade 86   1.mob.N.order 58   0.quest.N.order 56
  1.endscript 15   0.startscript 12   0.skill.N.{levelCondition,level,id,acquire} 12 each
  0.start_t 8  0.start 8  0.end_t 7  0.end 7  0.weeklyRepeat 6  1.failscript 4
  0.item.N.order 2  0.item.N.id 2  0.dayByDay 2
  1.npcSpeech.N.{script,order,id} 1 each   0.pop 1   0.interval 1
```

`lvmin` 317 + 89 = **406**, `job` **104**, `skill` 12x4 = **48**, `item`
330+329+135+2+2 = **798**. Those are `jobs.rs`'s four numbers to the unit, which is the
control that says this enumeration and that one are looking at the same thing.

**Not one of the 37 is `str`, `dex`, `int` or `luk`.** The requirement is not in `Quest.wz`.
**[L]**

### One correction to `jobs.rs`'s doc block

It says there is *"no `Act.<n>.job` key anywhere in the tree"*. There are **57**
`Act.1.item.N.job` cells and two `Act.1.item.N.jobEx`. The literal sentence is too strong.

They are a **job bitmask selecting which reward item a player gets**, not a job change. Quest
`10007` is the clearest:

```text
10007  Act 1.item.1.id 1082000   1.item.1.job  1     <- Warrior glove
10007  Act 1.item.2.id 1082054   1.item.2.job  2     <- Magician
10007  Act 1.item.3.id 1082057   1.item.3.job  4     <- Bowman
10007  Act 1.item.4.id 1082060   1.item.4.job  8     <- Thief
10007  Act 1.item.5.id 1082063   1.item.5.job 16     <- Pirate
```

**[L]** The conclusion `jobs.rs` drew is unaffected: **no quest in this client changes a job**,
so both advancements are the server's to perform. Only the sentence needs narrowing, and it is
recorded here rather than edited into a file this pass does not own.

---

## 5. What the advancement grants

| | value | label |
|---|---|---|
| job id | 110 / 120 / 130 / 210 / 220 / 230 / 310 / 320 / 410 / 420 | **[L]** |
| SP pool key | **2** for all ten - a job **TIER**, never a job id | **[D]** |
| SP owed | `skillpoints::entitlement(Tier::Second, level)` - a **TOTAL**, not an increment | The owner's rule |
| max HP gain | **0 points** | see below |
| max MP gain | **0 points** | see below |
| SP encoding forks? | **no**, for all ten | **[D]** |

### The pool key is a tier

`net::stats::tier_for_job` (`FUN_140286E90`, disassembled by hand). Every second job ends in
`0`, so `2 + (job % 10)` is **2** for all ten. `FUN_1402CB030` returns `0` for any key above
10, so putting a job id here would read an empty pool and grey the `+` button with nothing on
screen to say why. Controls in the test: tier 0 for job 0, tier 1 for job 100, tier 3 for 111.

### The SP encoding does not fork

`net::opcode::uses_extended_sp` accepts `x10..x12`, `x20..x22`, `x30..x32` in the 1xx/2xx
branches and `x10..x12`, `x20..x22` in 3xx/4xx. All ten second jobs are on the **same** side as
their first job, so a first->second advancement does not change the shape of the SP field and
the combined `0x007C` `session::gm::job_change_reply` already builds stays correct. **[D]**

The control that makes this worth asserting: job `101` genuinely *is* on the other side, and
sending the wrong shape desynchronises the whole packet rather than merely losing the points.
`jobs::sp_encoding_changes` is reused rather than reimplemented - two copies of one predicate
is how one of them gets missed.

### The HP/MP jump: this client supplies no number

**`MAX_HP_GAIN_POINTS` and `MAX_MP_GAIN_POINTS` are both `0`, and that is a decision, not
"unset".**

All 100+ images of `Etc_000.wz` were enumerated. The only job-related tables in it are
`FreeJobChange.img` (a *cash* job change gated at level 105 - `Meso.base 10000000`,
`Coin.base 5`) and `MakeCharInfo.img` (character creation). **There is no per-job and no
per-advancement HP/MP table in `Etc.wz`, `String.wz` or `Skill.wz`.** **[L]** - and those two
images are the positive control that the enumeration can see a job-related table when there is
one, which is the difference between a measured negative and a failed search.

So a non-zero value here would be a **second** unverifiable constant beside
`jobs::STAT_MINIMUM`, and the brief for this pass asked for that to be refused rather than
quietly chosen. The first advancement grants no HP/MP either: `gm_job` sends the job and the
SP table and nothing else. If the owner supplies a number it is one edit, in one named place.

---

## 6. What this client does NOT name: the ten job names

`String.wz` names the four *first* jobs and names **none** of the ten second jobs.

The negative has a positive control, which is what makes it worth reporting. The same grep of
`gm-handbook/questlines.txt`:

```text
  Swordsman     3        Fighter        0
  Magician     15        Spearman       0
  Archer        3        Wizard         0
  Rogue         3        Cleric         0
                         Crossbowman    0
                         Assassin       0
                         Bandit         0
                         Page           0
                         Hunter         7   <- "Scadur the Hunter", an NPC, not the job
```

Nor are they in `String.wz`'s other images (every hit there is an *item* name: `Black Hunter
Boots`, `Blue Wizard Robe`, `Bronze Crusader Helm`...), and nor are they in `MapleStory.exe`:
`Fighter`, `Spearman`, `Bandit`, `Crossbowman`, `Cleric`, `Sniper` and `Hermit` appear **zero**
times as ASCII or as UTF-16LE. Neither do `Swordsman`, `Archer` or `Rogue` - the executable is
simply not where job names live in this build.

**The only second-job words this client owns are the skill book names**, and seven of the ten
carry the job name as their leading word:

```text
  110  "Fighter Techniques"        ->  Fighter        [D]
  120  "Page's Path"               ->  Page           [D]
  130  "Spearman Techniques"       ->  Spearman       [D]
  310  "Hunter's Guide"            ->  Hunter         [D]
  320  "Crossbowman Guide"         ->  Crossbowman    [D]
  410  "Assassin Skills"           ->  Assassin       [D]
  420  "Bandit's Tricks"           ->  Bandit         [D]
  210  "Fire & Poison Basics"      ->  ???            [I]
  220  "Ice & Lightning Basics"    ->  ???            [I]
  230  "Holy Magic Basics"         ->  ???            [I]
```

`secondjob::NameSource` carries that split **in the type**, and the test asserts it in both
directions: an entry claiming `BookName` must satisfy `book_name.starts_with(job_name)`, and
an entry claiming `NotInThisClient` must **not**. So the enum cannot be set by habit.

Read from the archive with:

```
python tools/dump_names.py            # NOT this - it does items and maps
target\release\wz-dump.exe cat client-patched\Data\String\String_000.wz Skill.img
```

and the `bookName` lives at `<book>/bookName` for each of the 25 three-character keys.

---

## 7. THIRTEEN INVISIBLE SKILLS, and the one place `skilltable` is currently wrong

**`skilltable::book()` would hand a Fighter ten skills where the player may only ever have
eight.**

`SkillTable` does not parse the `invisible` column - it has no field for it - and `book()`'s
filter is `job != 0 && may_learn(...)`. At **first** job that is harmless: none of the 24
first-job skills is invisible, which is exactly why nothing has caught it. At **second** job it
is wrong for four of the ten books.

The enumeration, over all 176 skills in all 25 books:

```text
  skills in the archive                        176
  with a String.wz/Skill.img name              163
  with NO name                                  13
  with invisible = 1                            13
  unnamed but NOT invisible                      0
  invisible but NAMED                            0
```

**The two sets are identical.** **[L]** That is an equivalence with zero counterexamples in
either direction, which is a much stronger statement than either half alone -
`CLAUDE.md`'s *"different is a much weaker observation than the same"*.

The thirteen:

```text
  1101006 1101007   Fighter          1201006 1201007   Page
  1301006 1301007   Spearman         2101005           Wizard (Fire/Poison)
  3101005 3101006   Hunter           3201005           Crossbowman
  2111006 3111006 3211006            (third-job books, listed for completeness)
```

### What they are, and where the obvious explanation runs out

Four of the thirteen are the hidden half of a named skill, named by that skill's own
`extraSkillInfo` node:

```text
  2101004 Poison Breath  ->  extraSkillInfo/0 = { skill: "2101005", delay: "0" }
  3101004 Arrow Bomb     ->  3101005
  3111001 Inferno        ->  3111006
  3211001 Blizzard       ->  3211006
```

Poison Breath is the clearest case, and it is a warning about reading `Skill.wz` a level at a
time. Its own level-1 node is:

```json
{ "hs": "h1", "mpCon": 20, "mobCount": 1, "attackCount": 1, "bulletCount": 1 }
```

**No `mad`, no `damage`, no `dot`, no `mastery`, no `prop`** - and `common` is `null`, so
nothing is being merged in either. Yet its tooltip reads *"MP -20; Basic Attack 50; Mastery
level 1; 45% success rate; deals 10 Basic Attack over 5 sec"*. Every one of those numbers is on
**`2101005`**:

```json
{ "hs":"h1", "mpCon":20, "mad":50, "mastery":1, "prop":45, "dot":10,
  "dotTime":5, "dotInterval":1, "mobCount":4, "attackCount":1,
  "lt":{"x":-130,"y":-50}, "rb":{"x":130,"y":50} }
```

`2101005` also carries `processtype: -1` and `invisible: 1`. **[L]**

**The other nine are referenced from nowhere in `Skill.wz`.** `extraSkillInfo` produces exactly
four edges across the entire archive, so it explains 4 of 13 and **is not the discriminator**.
`invisible` is, and it is exact. Saying that out loud rather than generalising the four is the
point: `CLAUDE.md`'s *"a burst is not an object; it is a set, and it has to be enumerated like
one"*.

### What to do about it

`secondjob::HIDDEN_SKILLS` holds the thirteen, `secondjob::is_hidden` tests it, and
`SecondJob::grantable_skills()` filters the book through that one predicate. The test asserts
the constant equals the `invisible = 1` set in `gm-handbook/skills.txt` **exactly**, in both
directions, with a positive control that the file loaded first.

`crates/world/src/skilltable.rs` was off-limits to this pass. **The durable fix is a
`Skill::invisible: bool` there** (column 24, present on exactly those 13 rows and empty on
every other), so that `book()` and `may_learn()` agree with this without a second list.

---

## 8. Two places this build differs from the classic tree

Both **[L]**, both found by enumerating the `job` column rather than by checking a remembered
list - which is the same habit that caught `2001002` being Energy Bolt here where the classic
tree puts Magic Guard.

* **Power Knock-Back is a FIRST-job Archer skill.** The classic tree has it at `3101002`
  (Hunter) and `3201002` (Crossbowman). In this client **both of those ids are absent** and the
  skill is `3001003`, in book `300`. `crates/world/src/firstjob.rs` already carries it as one of
  the 24, gated on weapons `45`/`46`. The gaps in `HUNTER_BOOK` and `CROSSBOWMAN_BOOK` are the
  client's data, not a transcription slip, and the test asserts all three facts together.
* **Several second-job books carry skills the classic tree does not**: `3100001` /`3200001`
  *Amazon's Judgement*, `4100002` *Critical Recovery*, `4200001` *Nimble Recovery*, and
  `1101006`/`1101007`-style invisible pairs on all three Warrior books. This is a later
  MapleStory build wearing a classic skin. **Do not predict a skill id from memory here.**

### Weapon gates, for whoever plans the test run

From the `weapon`..`weapon4` columns, in the client's own 30..=47 numbering
(`itemId / 10000 - 100`), which `world::damage::WeaponClass` models:

```text
  110 Fighter      sword 30/40, axe 31/41          Rage is ungated
  120 Page         sword 30/40, blunt 32/42        Threaten is ungated
  130 Spearman     spear 43, polearm 44            Iron Will is ungated
  210/220/230      no weapon gate on any skill
  310 Hunter       bow 45                          Amazon's Judgement is ungated
  320 Crossbowman  crossbow 46                     Amazon's Judgement is ungated
  410 Assassin     claw 47                         Haste, Critical Throw/Recovery ungated
  420 Bandit       dagger 33                       Haste, Nimble Recovery, Steal ungated
```

**[L]** So a Warrior branch is testable with the weapon they already carry, and a Bowman or
Thief needs the right class of weapon before most of the book does anything.

---

## 9. WIRE IT LIKE THIS

**None of this is connected.** `crates/world/src/session/` was not touched. What follows is a
recipe, not a description of what happens today.

### 9.1 The NPC click

`session/npc.rs` handles a talk. The four instructor templates now answer **two** questions, so
the order matters:

```rust
use crate::{jobs, secondjob};

// 1. First advancement: a beginner at 511/313/221/411.
match jobs::advancement_for(&chr, npc_template) {
    jobs::Advancement::NotAnInstructor => {}          // fall through to step 2
    jobs::Advancement::AlreadyAdvanced { .. } => {}    // fall through to step 2
    other => return first_job_dialog(other),           // existing path
}

// 2. Second advancement.
match secondjob::advancement_for(&chr, npc_template) {
    secondjob::Advancement::NotAnInstructor => { /* ordinary NPC */ }
    // NOT a refusal. jobs::advancement_for already had its turn above.
    secondjob::Advancement::StillABeginner => { /* ordinary NPC */ }
    secondjob::Advancement::Choose { branch } => {
        // Ask which. `branch.choices` is 2 or 3 SecondJobs; put `job_name` on the buttons
        // and remember `branch.instructor_npc` so the answer can be validated.
    }
    refused => {
        // ALWAYS ANSWER. secondjob::refusal(&chr, npc_template) has the sentence.
        let _ = refused;
    }
}
```

`jobs::advancement_for` returning `AlreadyAdvanced` is the normal case for a first-job
character standing at their own instructor, so it **must** fall through rather than print.
`secondjob::refusal` returns `None` for `StillABeginner` on purpose, so a caller that just
prints whatever it gets cannot accidentally refuse a beginner.

### 9.2 The choice comes back

```rust
match secondjob::advancement_to(&chr, npc_template, chosen_job) {
    Err(refusal) => { /* say secondjob::refusal(...); do NOT change anything */ }
    Ok(grant) => { /* 9.3 */ }
}
```

`advancement_to` re-runs the whole decision. The chosen job arrives off a socket and nothing
upstream has to have validated it, so a job from another branch comes back as
`Err(WrongBranch)` rather than being granted.

### 9.3 Applying the grant

**Every effect hangs off the transition, not off the request** - `CLAUDE.md`'s Heena lesson.
Return early on the refusal; do not gate each effect separately.

```rust
let was = chr.job;
chr.job = grant.job;
store.save_character_progress(&chr)?;          // if this fails, send nothing

// One packet, exactly as gm_job does: mask bit 5 (job) plus the SP bit.
// grant.sp_encoding_changes is false for all ten, so Sp::Extended stays correct - but
// assert it rather than assuming, because the wrong shape desynchronises the packet.
debug_assert!(!grant.sp_encoding_changes);
out.push(self.job_change_reply(was, grant.job));
```

`job_change_reply` already computes both pools from the level via
`skillpoints::entitlement`, and `tier_for_job(110) == 2`, so **the second pool appears with no
change to that function**. Verify on the first run that the `2nd job` row shows up beside the
`1st job` one in the skill window.

`grant.max_hp_gain_points` and `grant.max_mp_gain_points` are `0`, so there is no HP/MP field
to add to the `StatChange`. If the owner supplies numbers, they go in the same packet
(`StatChange { max_hp: Some(..), max_mp: Some(..) }`) and the constants in `secondjob.rs` are
the one place to change.

### 9.4 A GM command to make it testable without the quest chain

`!job` already sets any job id, so the second advancement is *reachable* today - but it skips
every check and, per its own doc block, moves no ability points. A `!job2 <110|120|...>` that
routes through `secondjob::advancement_to` would exercise the real decision. Cheaper still:
`!job 100`, `!level 30`, then click 511, once §9.1 exists.

### 9.5 What is still missing

* **Spending is still not persisted.** `skillpoints` says so: the amount is computed from the
  level, nothing records what has been spent, so points come back on the next advancement.
  Adding a second pool doubles the surface of that. **Say it when reporting** - a player who
  spends and sees them return will file it as a bug.
* **`skilltable::book()` will offer the thirteen invisible skills** to a second-job character.
  §7. `!learn` on a Fighter would report ten and grant two the client cannot draw.
* **No second-job skill has a cast handler.** `firstjob.rs` classifies the 24 first-job skills
  and `session/skills.rs` acts on them; nothing equivalent exists for the 66.
* **The `Test of Qualification` hidden field and its `q20002s` script do not exist**, so the
  client's own route to the advancement is not walkable.

---

## 10. How to re-derive every number here

With **the repo as the working directory** (`CLAUDE.md`: the scratchpad shadows the real
tools):

```
cd C:\MapleCW
python tools\dump_skills.py                    # gm-handbook\skills.txt
python tools\dump_quests.py                    # gm-handbook\questlines.txt, quests.json
python tools\dump_npcstrings.py                # gm-handbook\npcstrings.txt
python tools\dump_names.py                     # gm-handbook\maps.txt, items.txt
python tools\dump_portals.py                   # gm-handbook\npcs.txt, portals.txt

target\release\wz-dump.exe cat client-patched\Data\String\String_000.wz Skill.img
target\release\wz-dump.exe cat client-patched\Data\Skill\Skill_000.wz 210.img
target\release\wz-dump.exe tree client-patched\Data\Etc\Etc_000.wz 1
```

`cargo test -p world --lib secondjob` asserts the table against the generated files and
degrades to a no-op when `gm-handbook/` has not been generated. **That degradation was itself
tested**: three data claims were deliberately corrupted (a Cleric skill id, one hidden-skill
id, the marble count) and all four file-backed tests failed. A green run on this machine means
the files were read, not that the check was skipped.
