# First job advancement — what the client already does, and what is left

**Written 2026-08-21. No Ghidra** (another agent held the project lock). Instruments: the
repo's `tools/listing.py`, `tools/dump_va.py`, `tools/dataref.py`, `tools/callers.py`,
`target/release/wz-dump`, and the generated `gm-handbook/*.txt`. Every instrument was shown
to find a positive control before any negative below was written down — §10.

Labels: **[L]** read off a listing, this client's WZ, or a capture; **[D]** derived from two
or more [L] facts; **[I]** inferred — a policy nothing on this machine can confirm.

**Nothing authenticates.** The channel socket carries no credentials, and none of what
follows changes that: an advancement is granted on the say-so of whoever holds the socket.

---

## 0. Answer up front

| question | answer |
|---|---|
| What is the packet that changes a character's job? | **`0x007C` `StatChanged`, mask bit 5 (`0x20`), body `u16 job, u16 subJob`.** Already decoded, already in `crates/net/src/stats.rs` as `bits::JOB`. **[L]** Goal E says this is "still to be found"; it was found before goal E was written and nobody joined the two up. |
| Is that enough, or is a second packet needed? | **It is enough, and it is more than enough.** The `0x007C` handler `FUN_142d54780` **plays the JobChanged animation and sound by itself** when bit 5 is set and the resulting job is non-zero — `142d55bce`..`142d55d59`. **[L]** Same shape as the level-up animation being client-side off `bits::LEVEL`. |
| So do we send `0x02D1` effect 14 as well? | **No.** It would double the fanfare — and it is also a **5-byte body, not 1**: effect 14's arm at `0x14278e040` reads **two `u16`s` before it plays anything, where effect 15's arm reads none. A copy-paste of `quest_clear_local()` would ship a short packet. **[L]**, §4. |
| Are the `Test of Qualification` quests the first advancement? | **No — they are the SECOND, at level 30.** `Check.0.job.0` is `100`/`200`/`300`/`400` and `Check.0.lvmin` is `30`, and the client's own quest text says *"the 2nd job advancement"* in as many words. **[L]**, §2. |
| Then which quest drives the first advancement? | **None. There is no such quest.** Enumerated, not searched: **no `Act.<n>.job` key exists anywhere in the 322 quests**, so no quest in this client can change a job even in principle. **[L]**, §3. |
| So what drives it? | **A server-side NPC script**, exactly like `q1002s` — one level up. The client sends `0x00F2` and the whole conversation is `0x055B`/`0x00F3`, which already works. **[D]**, §5. |
| Where do the instructors stand? | **Not in the towns.** 511 is on **10004003**, 313 on **10002003**, 221 on **10001051**, 411 on **10003003**. **[L]**, §6. This is the detail that costs a manual launch. |
| Can a level-10 beginner reach 35 in a stat? | **On paper yes** (25 rolled + 45 AP = 70 points). **In this server as this was written, no** — there is no GM command that sets a stat, and a sibling agent's AP-allocation handlers exist but are **not dispatched**. §9.1, which says how to re-check rather than asking anyone to trust it. |
| Is the 35 verifiable? | **No, and nothing will ever catch it being wrong.** **[I]**, §7. |

---

## 1. The correction, first, because everything else follows from it

`STATUS.md`'s goal E and `research/quest-scripts.md` §4 both point at quests
`20002 / 20102 / 20202 / 20302` (*Test of Qualification*) and
`20003 / 20103 / 20203 / 20303` (*Proof of Qualification*) as the advancement path, and §4
calls the second four "the cheapest next ones". They **are** cheap and they **are**
advancement quests — but they are the **second** advancement, at level 30.

From `gm-handbook/questlines.txt`, **[L]**, all four families identically:

```
20002  Check  0.job.0    100        <- you must ALREADY be a Warrior
20002  Check  0.lvmin    30
20002  Check  0.npc      514        <- the deputy, not Dances with Balrog
20003  Check  1.npc      511
20003  Act    1.exp      3150
```

And the client says it in words, twice per line of the chain:

> `20001 Say 1.0` — *"…you came all the way here to take the test and make the **2nd job
> advancement** as a Warrior?"* **[L]**
>
> `20003 QuestInfo 1` — *"I received #b#t4031018##k for successfully passing the #b#p514##k's
> test. I should return to #b#p511##k in #b#m10004000##k now."* **[L]**

The whole `200xx` family is one chain per class: `20000` (self-start at 30) → `20001` (carry
a letter to the deputy) → `20002` (collect 30 Black Marbles) → `20003` (bring the proof back
to the town instructor). Four chains, `100`/`200`/`300`/`400` gated, `lvmin 30` throughout.

**This matters because acting on the original reading would have advanced a level-10
beginner through a level-30 quest**, and the failure would have looked like the feature
working.

---

## 2. Which NPC set does which advancement — settled

The two sets in `STATUS.md` goal E are not alternatives; they are **different jobs in the
same chain**. From every `Check.<n>.npc` row in all 322 quests, **[L]**:

| template | name | role | quests it appears on |
|---:|---|---|---|
| **511** | Dances with Balrog | town instructor, Warrior | `20000.0/1`, `20001.0`, `20003.1`, `12126.1`, `12127`, `12128.0`, `80028.1`, `80029.0` |
| **313** | Grendel the Really Old | town instructor, Magician | `20100.0/1`, `20101.0`, `20103.1`, `106xx`, `1212x` |
| **221** | Athena Pierce | town instructor, Bowman | `20200.0/1`, `20201.0`, `20203.1`, `1213x`, `50601x` |
| **411** | Dark Lord | town instructor, Thief | `20300.0/1`, `20301.0`, `20303.1`, `1213x`, `50611x` |
| 514 | *Warrior Job Instructor* | **2nd-job examiner** | `20001.1`, `20002.0/1`, `20003.0` |
| 319 | *Magician Job Instructor* | 2nd-job examiner | `20101.1`, `20102.0/1`, `20103.0` |
| 227 | *Bowman Job Instructor* | 2nd-job examiner | `20201.1`, `20202.0/1`, `20203.0` |
| 424 | *Thief Job Instructor* | 2nd-job examiner | `20301.1`, `20302.0/1`, `20303.0` |
| 800003 / 800004 | — | **no quest references either** | none |

So **the named four (511/313/221/411) are the right ones for the first advancement**, the
deputies are second-job examiners and must not be treated as instructors, and the
`800003`/`800004` pair is referenced by no quest at all.

The client's own signpost quest agrees independently. `Quest.wz/QuestData/10001.img`
("Phil's Call", NPC 101 in Lith Harbor, `Check.0.job.0 = 0`, `Check.1.lvmin = 10`) **[L]**:

> *"Now that you've reached level 10, you can choose to advance to one of the following
> classes: #rSwordsman#k, #rMagician#k, #rArcher#k, or #rRogue#k."*
>
> *"To become a Swordsman, you need to go to #m10004000#; to become a Magician, you need to
> head to #m10002000#; to become an Archer, you need to go to #m10001000#; and to become a
> Rogue, you need to make your way to #m10003000#."*

That is the client naming all four jobs, all four towns and the level, in one place. It is
the strongest [L] evidence in this document for the *shape* of the feature.

> **One typo in the client, so nobody "fixes" it in our direction.** `10001`'s
> `QuestInfo.2` says *"a Thief in #b#m10004000##k"* — Perion. The `Say.1.yes.1` line above
> says Kerning City (`10003000`), and the Dark Lord is on a Kerning City sub-map. The
> journal line is wrong and the dialogue is right. **[L]** both.

---

## 3. No quest in this client can change a job — enumerated

`CLAUDE.md`'s rule is enumerate before you filter, so this is the **whole key space** of
`Act` and `Check`, digits collapsed, over all 322 quests. **[L]**

```
Act:    772 N.item.N.id      772 N.item.N.count   310 N.exp        256 N.money
        180 N.item.N.prop    131 N.item.N.order   119 N.nextQuest  115 N.item.N.potentialGrade
         86 N.citizenshipContr.town                57 N.item.N.job
         50 N.citizenshipContr.amount              36 N.citizenshipContr.amountFormula
         23 N.item.N.gender   18 N.skill.N.id      18 N.skill.N.exp  15 N.pop
          2 N.item.N.jobEx
                                        <- 17 shapes. There is no `N.job`.

Check:  640 N.npc   406 N.lvmin   332 N.item.N.id   329 N.item.N.count
        215 N.quest.N.state  215 N.quest.N.id  137 N.item.N.order  110 N.order
        104 N.job.N   87 N.mob.N.id   87 N.mob.N.count   86 N.citizenshipTown
        ... 32 shapes in total, including endscript(15), startscript(12), failscript(4) ...
```

**`Act.<n>.item.<n>.job` exists 57 times and is not a job change** — it is a *filter* on a
reward item (which class gets which item), the sibling of `.gender` and `.prop`. The two
`.jobEx` rows are the same thing.

**`Check.<n>.job.<n>` exists 104 times and is a precondition, never an effect.** Its full
value distribution is `0` (21 uses — beginner-only quests) plus every explorer job id from
`100` to `436`. Not one of them sits under `Act`.

**Instrument control**: the same one-line scan enumerates 17 `Act` shapes and 32 `Check`
shapes including four that occur exactly once (`N.pop`, `N.interval`,
`N.npcSpeech.N.script`, `N.npcSpeech.N.order`). A scan that can see a key with one use is
not a scan that missed a key with dozens. **The negative is verified, not merely empty.**

Cross-checked against the loader: `crates/world/src/config.rs`'s `read_quest_rows` matches
on `QuestInfo`, `Check 0.npc`, `Check 1.npc`, `Act *.nextQuest`, `Act *.item.*`,
`Act 1.exp` and `Say` — there is no arm for a job because there is no key for one.

### 3.1 The one quest that comes closest, and why it is not it

`10001` "Phil's Call" is the only quest in the tree with both `Check.0.job.0 = 0` and a
`lvmin` of 10. **[L]** It is the signpost quoted in §2: it *tells* you about the
advancement, offers a paid teleport to the town of your choice, and pays 171 exp, 5 potions
and 204 mesos. Its `Act` has no job row either. **It is the tutorial for the feature, not
the feature.**

---

## 4. The packet — `0x007C` bit 5, and the trap next door

### 4.1 What the bit is

`crates/net/src/stats.rs`, already in the tree and already tested **[L]**:

```rust
pub const JOB: u32 = 0x0000_0020;   // `u16 job`, then `u16 subJob` - two values, one bit
```

Written by `StatChange::build` at `1402cbbf8` / `1402cbc0f`; `subJob` lands plainly at
`record+0x10c`, `job` at the obfuscated `record+0x33`.

### 4.2 The client draws the fanfare itself — this is the finding

`FUN_142d54780` is the `0x007C` handler. (Pinned: `stats.rs` documents `0x142d549be` as the
site of the "secondary" byte's one use, and `tools/listing.py` puts that address inside this
function.) Its tail, **[L]**, `tools/listing.py`:

```text
142d55b52  mov   r12d, [rbp-0x60]      ; the packet's stat MASK
142d55b59  and   r15d, 0x30            ; LEVEL(0x10) | JOB(0x20)
142d55b5d  je    0x142d55efd           ;   neither set -> skip the whole block
           ... four UI-refresh calls, each null-guarded ...
142d55bce  test  r12b, 0x20            ; the JOB bit specifically
142d55bd2  je    0x142d55efd           ;   not set -> skip
142d55bd8  mov   edx, [r13+0x37]       ; subJob
142d55bdc  lea   rdi, [r13+0x33]       ; job
142d55be0  call  0x1401ab420           ; the obfuscated getter - reads the job BACK OUT
142d55be8  test  ax, ax
142d55beb  je    0x142d55ebb           ;   job == 0 -> NO fanfare
142d55bf1  mov   r14, [rip+0xcf1358]   ; = [0x143A46F50] -> L"Effect/BasicEff.img/JobChanged"
           ...
142d55d46  call  0x140e16070           ; play the animation
142d55d52  mov   rcx, [rip+0xcf273f]   ; = [0x143A48498] -> L"JobChanged"
142d55d59  call  0x1429f14c0           ; play the sound, volume 0x64 = 100
```

Three consequences, all **[L]** except where marked:

* **A single `0x007C` with bit 5 set is the whole feature on the wire.** The animation and
  the sound are the client's own reaction to the bit.
* **`job == 0` suppresses the fanfare**, which is the right behaviour and means a "revert to
  beginner" would be silent by design rather than by accident.
* `0x1401ab420` is the **same getter** `research/charstat-layout.md` §129 names for the SP
  fork. The client re-reads its own record rather than trusting the packet field, so the
  record write has to land before this — which it does; the bit's value is written earlier in
  the same function.

After the sound the handler reads the job a **third** time at `142d55d94` and performs two
hash-map operations (`FUN_1401c21c0`) against tables adjacent to the `Skill.wz` property
names `minLevel` / `maxLevel` (`0x143298058`) and a prime-sized bucket table
(`0x1434956B0`). **[D]** that this is the client rebuilding its skill state for the new job;
the detail is **[I]** and was not chased, because nothing we send changes it.

### 4.3 The animation node exists, unlike the quest one

`Effect_000.wz/BasicEff.img` decodes to **40** nodes and `JobChanged` **is one of them**
**[L]** — verified with `wz-dump cat`, full node list in §10. `Sound_001.wz/Game.img` has
**39** nodes and `JobChanged` is one of them **[L]**.

That is the opposite of the quest-clear case: `research/quest-complete-effect.md` records
that `QuestClear` is *the only* one of the seven `Effect/BasicEff.img/*` paths this client
holds whose node was cut, so effect 15 gives sound and no picture. **Effect 14 has both.**
So a job change should be visibly *and* audibly obvious on screen, which is what makes the
test in §11 discriminating.

### 4.4 The trap: `0x02D1` effect 14 is **five** bytes, not one

Reading `research/quest-complete-effect.md` and reaching for `stats::user_effect_local(14)`
is the obvious next move and it would ship a short packet — the failure that killed this
client twice.

Both effects share the first switch's default arm (byte table `0x142791300`, index
`effect-8`: index 6 → `0x18` = 24 and index 7 → `0x18` = 24; dword table `0x14279129c`
index 24 → `0x14278bd20`). **[L]** They then split on the second switch, table
`0x142791348`:

| index | arm | packet reads inside the arm | strings it loads |
|---:|---|---|---|
| 14 | `0x14278e040` | **`u16` at `14278e043`, `u16` at `14278e04e`** | `Effect/BasicEff.img/JobChanged`, `JobChanged` |
| 15 | `0x14278e11d` | **none** | `Effect/BasicEff.img/QuestClear`, `QuestClear` |

**Control for the table read**: index 15 resolves to `0x14278e11d`, which is exactly what
`crates/net/src/questeffect.rs` documents from an independent Ghidra session. The two agree
without sharing a method — one read the table statically off disk, the other read the
listing. **[L]**

What the two `u16`s are: they are loaded into `ebx`/`edi` and handed to
`FUN_1429dc320(user, ebx, edi)` **only if** a virtual predicate `[[r15]+0x60]` returns
non-zero; the animation and sound then play unconditionally either way. `FUN_1429dc320` has
exactly two callers (`tools/callers.py`: this arm and `0x14285f8e0`, which is the *other*
JobChanged site) and no `.pdata` entry of its own, so it was not disassembled. **Their
meaning is not established** — most likely `(job, subJob)`, by position and width, but that
is **[I]** and unchecked.

**The recommendation is not to send `0x02D1` at all**, which makes the question moot: the
`0x007C` already fires the same animation and the same sound, and two of them would stack.
If a remote-player fanfare is ever needed (`0x02AF`, effect 14, `u32 charId` first) the two
`u16`s have to be settled first — this server cannot push into another session anyway.

### 4.5 One thing to fix in passing, in someone else's file

`crates/net/src/stats.rs`'s doc for the `secondary` byte says the mask `0x40030` covers
"level, job, exp, meso". `0x40030` = `0x10 | 0x20 | 0x40000` = **LEVEL | JOB | MESO**;
`EXP` is `0x10000` and is *not* in it. The code is right and the sentence lists one field
too many. Not edited — that file is not mine.

---

## 5. The conversation: it is an NPC script, and it already works

`research/npc-click.md` §2 established that **the client chooses `0x0151` (quest) over
`0x00F2` (plain talk) entirely by itself**, off `Quest.wz`, and that the per-state quest
arrays are filled by `FUN_141e40010` → `FUN_140711d70`, **the requirement checker**. **[L]**

So for a level-10 beginner clicking Dances with Balrog: every quest with `Check.0.npc = 511`
is gated on `job.0 = 100` and `lvmin 30` (or `lvmin 55`, for the `1212x` sage chain), the
requirement checker rejects all of them, the array comes back empty, and the click leaves by
`141e3da78` → **`0x00F2`, site C**. **[D]**, from [L] gates and an [L] branch.

That is the same packet Robin and Lucy produce, `Session::on_npc_click` already answers it,
and `net::script::npc_say` / `npc_ask` already draw both box types. **The machinery for the
first job advancement exists in full; only the words and the decision are missing.** That is
the same conclusion `research/quest-scripts.md` §5 reached for `q1002s`, one level up.

### 5.1 The one shape that does not exist yet

`Session::say_line` only offers an Accept/Decline box when `convo.quest_id.is_some()`:

```rust
let branches = last && !on_branch && convo.quest_id.is_some() && self.has_branch(&convo, "yes");
```

A job advancement has **no quest id** — §3 — so as the code stands an instructor
conversation can only ever draw plain `Say` boxes. The yes/no needs a conversation kind that
is not quest-shaped. §8 says exactly how, and it is small.

### 5.2 The named blind spot in the `0x00F2` prediction

Quoted rather than summarised, because `CLAUDE.md` requires a hedged negative to name its
own blind spot before anything is built on it:

> `FUN_1428de280`'s tail is `if (FUN_1401d3220(tid)) -> open a UI` **else** `0x00F2` site D,
> and inside `FUN_141e3c5d0` the branch at `141e3c72b` is *"if `local` is a non-empty C
> string → `0x00F2` site A, **else return silently**"*. `FUN_1401d3220` was not
> disassembled, and neither was `template->[0x178]`.

So there are two client-side paths on which **clicking an instructor sends no packet at
all**. Both are client-side and neither is a server bug. This is why §11's test looks at
`world.log` for the `0x00F2` *first* and at the screen second.

---

## 6. Where the instructors actually are — and it is not the towns

From `Map.wz`'s `life` nodes via `gm-handbook/npcs.txt`, cross-referenced with
`gm-handbook/maps.txt` and `gm-handbook/fields.txt`. **[L]**

| job | instructor | template | **spawns on** | map name | the town its text names |
|---|---|---:|---:|---|---:|
| Warrior 100 | Dances with Balrog | 511 | **10004003** | Warriors' Sanctuary | 10004000 Perion |
| Magician 200 | Grendel the Really Old | 313 | **10002003** | Magic Library | 10002000 Ellinia |
| Bowman 300 | Athena Pierce | 221 | **10001051** | Bowman Instructional School | 10001000 Henesys |
| Thief 400 | Dark Lord | 411 | **10003003** | Thieves' Hideout | 10003000 Kerning City |

Every one of those eight ids is in `gm-handbook/fields.txt`, so `!map` will accept them —
checked, because `map_exists` used to be fail-open and `!map` now refuses outright.

**`!map 10004000` finds nobody.** Each instructor is exactly one map inside their town, and
in the live game you walk through a door to reach them. `crates/world/src/jobs.rs` carries
this as a *test* rather than a comment, because getting it wrong costs a launch.

### 6.1 The instructors have almost no words

`String.wz/Npc.img` via `gm-handbook/npcstrings.txt`, complete, **[L]**:

```
511  d0/d1/idle0/idle1/info0/info1   "Those who want to become a warrior, come see me..."
                                     "Do you want to become a warrior?"
313  d0/d1/idle0/info0               "All who desire to become a magician, talk to me..."
221  d0/d1/idle0/info0               "Those who want to become a bowman... Talk to me..."
411  d0/d1/idle0/info0               "Those that want to be a thief, come..."
```

That is the entire repertoire — four to six lines each, and every one an idle line. **There
is no advancement dialogue for these NPCs anywhere in the client**, in the English tree or
the Spanish overlay. Their `d0` is enough for the *first* box of a conversation and nothing
more; everything after it has to be ours, and §8 keeps it to three sentences rather than
inventing a script.

---

## 7. The 35 — [I], and permanently unfalsifiable here

The owner, 2026-08-19, flagging their own source:

> *"Each job has a pre-requisite, which I assume is widely available on the internet.
> Magician 35 INT, Warrior 35 STR, Thief 35 LUK, Bowman 35 DEX."*

`STATUS.md` goal E already records the verified negative and §3 above re-ran it from
scratch: there is no `str`/`dex`/`int`/`luk` `Check` key in any of the 322 quests, and the
enumeration that says so also finds 32 other `Check` shapes down to ones with a single use.

**The requirement is not in `Quest.wz` and it is not enforced by any quest, because there is
no quest.** The client cannot contradict this number and will never catch it being wrong.

So it is one constant — `world::jobs::STAT_MINIMUM` — with the stat-to-job pairing beside it
in `FIRST_JOBS`, and a test whose only job is to make "one edit to change" true. The level
gate, `LEVEL_MINIMUM = 10`, is better off: it is **[D]**, corroborated by quest `10001`'s
own `Check.1.lvmin = 10` and its text.

---

## 8. WIRE IT LIKE THIS

**Nothing below is wired.** `crates/world/src/jobs.rs` exists, compiles and passes 12 tests;
**no Rust calls it**, and `crates/world/src/lib.rs` has no `mod jobs;` line — several agents
are adding modules and the coordinator owns every `mod` line. This is `CLAUDE.md`'s "built
is not wired" state, declared rather than hidden.

### 8.0 The one line that makes the module exist

```rust
// crates/world/src/lib.rs, in the alphabetical run beside `fields` and `footholds`
pub mod jobs;
```

### 8.1 The packets, in order

For Dances with Balrog, NPC template **511**, map **10004003**:

```text
1.  client -> 0x00F2   <u32 npcObjectId> <i16 x> <i16 y> <u32 tail>       12 bytes
                       (NOT 0x0151 - 511 has no quest a beginner qualifies for, §5)

2.  server -> 0x055B   npc_say(511, <line 0>, prev=0, next=1)
3.  client -> 0x00F3   ... action 01

4.  server -> 0x055B   npc_ask(511, "Do you want to become a Swordsman?", quest_flavoured=true)
                       message type 0x10, draws BtQYes / BtQNo
5.  client -> 0x00F3   00000000 10 01                          6 bytes -> YES
                       (00 = No -> say the decline line ; FF = closed -> send nothing)

6.  server -> 0x007C   StatChange { job: Some((100, 0)), .. }
                       THE WHOLE JOB CHANGE. The client plays Effect/BasicEff.img/JobChanged
                       and the JobChanged sound by ITSELF - §4.2. Do NOT also send 0x02D1.
    server -> 0x055B   npc_say(511, <the congratulation>, prev=0, next=0)
7.  client -> 0x00F3   ... action 01                           -> conversation ends
```

### 8.2 The three edits, and none of them is a new packet

1. **`crates/world/src/lib.rs`** — the `mod` line in §8.0. Coordinator's.

2. **`crates/world/src/session/npc.rs`** — in `on_npc_click`, after the shop check and
   before the generic one-line conversation:

   ```rust
   if let Some(replies) = self.offer_job_advancement(template, &chr) { return replies; }
   ```

   `offer_job_advancement` is: `jobs::advancement_for(&chr, template)`;
   `NotAnInstructor` → `None` (fall through, do **not** refuse);
   any refusal arm → one `npc_say(template, jobs::refusal(..).unwrap(), false, false)` and
   **clear the conversation** — a refusal is a complete answer, and `CLAUDE.md`'s first rule
   is that an unanswered packet freezes the whole UI;
   `Eligible` → `npc_say(template, first.idle_line, false, true)` then, on the `0x00F3`, the
   `npc_ask`. The instructor's `idle_line` is the only [L] sentence available, so use it as
   the opener rather than writing a new one.

3. **`crates/world/src/session/mod.rs`** — the `Conversation` struct needs a third kind. It
   currently distinguishes "quest" from "no quest" by `quest_id: Option<u32>`, and
   `say_line`'s `branches` test is gated on `quest_id.is_some()` (§5.1), so a job
   advancement drawn today can never produce an Accept box. The smallest correct change is
   an explicit field — e.g. `offer: Option<JobOffer>` carrying the job id — tested in
   `say_line` beside `quest_id`, and read in `on_script_reply`'s `awaiting_yes_no` branch
   next to `accept_quest`.

   **Do not reuse `quest_id` with a fake quest number.** `accept_quest` calls
   `record_quest_start`, which writes a `quest_state` row; a synthetic id would put a
   nonexistent quest in the journal and in the database.

4. **`crates/world/src/session/combat.rs`** or wherever the coordinator prefers — the commit
   itself:

   ```rust
   chr.job = job;                      // 100 / 200 / 300 / 400
   self.store.save_character_progress(&chr)?;   // BEFORE the packet
   let mut change = net::stats::StatChange::default();
   change.job = Some((job, 0));
   // Nothing else is needed. Do NOT set `sp` here.
   ```

   **Why `sp` is deliberately absent**: `bits::SP`'s width forks on the job the client reads
   back out of its own record, and job `0` and all four first jobs are on the **same** side
   of that fork (all extended). `jobs::sp_encoding_changes(0, 100)` is `false`, and a test
   pins it. Setting `sp` here would mean choosing an `SpPool::job_level`, whose meaning
   `stats.rs` says outright is **not established**. Awarding SP is a separate decision; §9.

   **Do not send `0x02D1`.** §4.2 and §4.4.

### 8.3 What must NOT be done

* **Do not attach a job change to quest `20003` / `20103` / `20203` / `20303`.** Those are
  second-job (§1), there is no second-job table in this server, and `Act` has no job key to
  hang one on. The overlay rows added to `data/quest-scripts.txt` are dialogue only, and the
  file says so in a header nobody can miss.
* **Do not treat 514 / 319 / 227 / 424 as instructors.** `jobs::first_job_at` refuses them
  and a test pins the refusal.
* **Do not make advancement reversible.** The client's own text: *"a job advancement cannot
  be undone once made"* **[L]**. `Advancement::AlreadyAdvanced` is a refusal.

---

## 9. The two things that block a real end-to-end test

### 9.1 A level-10 beginner cannot reach 35 in any stat on this server

The arithmetic is fine. A character is created with a **client-rolled** spread — the create
request carries `strength/dexterity/intelligence/luck` and `crates/login` stores them
unchanged (`CreateCharacterRequest::character`) — and `STATUS.md` records two captures whose
four stats totalled **25** **[L]**. `world::expcurve::LevelGains` awards **5 AP a level**,
so level 10 carries `25 + 9*5 = 70` points. Even a stat that rolled to the floor clears 35.

**But nothing can spend them — as of this pass, and that is about to change.** The GM
command table is `map, item, exp, heal, exprate, mesorate, droprate, setrates, rates, help`
— **there is no command that sets a stat**, confirmed by reading the `match` in
`crates/world/src/session/gm.rs`.

> **A sibling agent is landing AP allocation right now, and it is NOT yet wired.** As this
> was written, `crates/world/src/session/ability.rs` and `crates/net/src/abilityup.rs` are
> both **untracked** and `crates/world/src/session/mod.rs` contains no reference to either —
> a grep for `ability` / `0x0138` / `0x0139` in the dispatcher returns nothing. So the
> handlers exist and nothing calls them: `CLAUDE.md`'s "built is not wired", observed rather
> than assumed. That file also corrects `STATUS.md` item 2 in passing — a plain `+` click
> sends **`0x0138`**, not `0x0139`, and answering only the bulk opcode still looks broken.
>
> **Do not take this section as a standing blocker.** The moment that agent's work is
> dispatched from `mod.rs`, `!` needs no GM command at all: the owner can allocate AP in the
> client's own stat window and the 35-stat gate becomes testable. Re-check the dispatcher
> before believing this paragraph — it is a snapshot of a directory two agents were writing
> to at once.

**What is true either way**: spending a launch on "does the prerequisite work" *before* the
job change itself is confirmed would produce a refusal box and nothing else. That ordering
is the whole point of writing this section down, and it does not depend on when AP lands.

### 9.2 SP: goal D's 1-vs-3 ambiguity is currently moot

`STATUS.md` goal D asks whether "under level 10" means the level reached (9→10 gives 3 SP)
or the level left (9→10 gives 1).

**Neither branch exists.** No SP is awarded anywhere: `expcurve.rs` says so in as many words
(*"SP is deliberately not awarded"*), `LevelGains` has only `ap`, `max_hp`, `max_mp`, and a
grep for `Sp::` / `sp:` across `expcurve.rs` and `session/combat.rs` finds nothing. There is
no comparison to flip.

**Recommendation: leave it, and decide it when SP is first awarded — which is here.** Job
advancement is the natural place, because that is where `SpPool::job_level` finally has to
mean something, and `stats.rs` states plainly that its meaning is not established. Deciding
the 1-vs-3 rule before deciding what a pool entry contains would be settling the easy half of
a question whose hard half is still open.

---

## 10. Instrument notes — every control, run before its negative

* **`tools/listing.py`** was verified against its own documented control before any listing
  below it was believed: `python tools/listing.py 0x140304100 | grep READ` printed
  `140304138 raw`, `140304144 u8`, `140304183 u8` and the `u16` run, exactly as the tool's
  own header requires.
* **`tools/dump_va.py`** was verified by *reproducing a number from another session*: dword
  table `0x142791348` index 15 → `0x14278e11d`, which is what `crates/net/src/questeffect.rs`
  documents from a Ghidra session this one could not run. Only after that was index 14 read.
* **Everything was run with the repo as the working directory**, so the scratchpad's stale
  `reads.py` / `listing.py` / `callers.py` copies could not shadow `tools/`.
* **`gm-handbook/fields.txt` nearly produced a confident wrong answer, from my own hand.**
  The first check reported that **all nine** instructor and town maps were "NOT IN
  fields.txt" — because the file is one bare id per line and the grep pattern demanded a
  comma or tab after it. Re-run with `grep -x` and a `grep -cx '1'` control, all nine are
  present. A `!map` guard reported as broken on that basis would have sent the next person
  hunting a bug that does not exist. Written down because it is exactly the shape
  `CLAUDE.md` warns about and it happened inside the pass that quotes the rule.
* **`Effect_000.wz/BasicEff.img`, all 40 nodes**, so the `JobChanged` positive is a full
  listing and not a hit: `Assaulter, Buff, BuffIconEffect, CameraMode, CitizenshipGet,
  CitizenshipGradeUp, CraftingLevelUp, CraftingUnlock, DoubleJump, EmptyCanvas, Enchant,
  Flying, ItemSkill, JobChanged, LevelUp, LevelUp2, NoBlue0/1, NoCri0..3, NoGreen0/1,
  NoKite0/1, NoProduction0/1, NoRed0..3, NoViolet0/1, PvpLevelUp, Summoned, Teleport,
  Transform, TransformOnLadder, archerDoubleJump`. `QuestClear` is absent, which reproduces
  `research/quest-complete-effect.md`'s finding independently and is the control for the
  `JobChanged` presence.
* **The quest-key enumeration was written to print the whole key space**, not to search for
  `job`. Had it been a search, `Act.<n>.item.<n>.job` (57 uses) would have matched and the
  answer would have been a confident, wrong yes.
* **`crates/world/src/jobs.rs` was compiled outside `cargo`** — `rustc --test` against
  `crates/net`'s rlib — precisely because six agents share one `target/` and a `cargo`
  result taken now is not evidence. 12 tests, 12 passed, 0 failed. It is not in `lib.rs`, so
  a workspace build cannot see it either way.

---

## 11. THE SINGLE DISCRIMINATING CLIENT TEST

**One variant, one launch.** It does not need the dialogue, the instructors, the maps, or
the 35-stat gate — all of which are blocked on §9.1 or on unwritten code. It tests the one
thing everything else rests on: **does `0x007C` bit 5 change a character's job?**

### What to add first (server side, no client run)

A GM command, in `crates/world/src/session/gm.rs`:

```
"job" => self.gm_job(arg),      // !job 100  ->  store chr.job, then one 0x007C with bit 5
```

Refuse anything that is not `0`, `100`, `200`, `300` or `400` (`jobs::is_first_job`), and
refuse it with an *answer*, not an error. Set **only** `change.job = Some((job, 0))` —
nothing else, so the run has exactly one variable.

### Then, from an **elevated** window

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

Enter the world, then type **`!job 100`** in the chat box.

### What to watch for, and what each outcome means

| what the owner sees | what it means |
|---|---|
| **a burst of effect over the character AND a fanfare sound** | `0x007C` bit 5 works and the client owns the whole presentation. Goal E's open question is closed and the rest of the feature is dialogue. This is the expected result. |
| **sound but no picture** | the packet worked; `BasicEff.img/JobChanged` failed to resolve. Unexpected — the node is present (§4.3) — and it would mean the animation call needs more than a name. Not a blocker. |
| **picture and sound, but the stat window still says Beginner** | the effect fired off the mask while the record write did not land. Look at `subJob`: it is the only other field the bit carries. |
| **nothing at all, and `world.log` shows the `0x007C` going out** | the mask arithmetic is wrong, or `secondary`/`context_flag` suppressed the refresh. `142d55b59 and r15d,0x30` is the first gate — check the mask byte on the wire against `0x20`. |
| **nothing, and no `0x007C` in `world.log`** | the GM command did not run. `!job` is not in the table, or the chat never arrived — `grep 0x00E7 world.log` settles it, as it did for `!map 45`. |
| **the client freezes** | not this packet: `0x007C` is not a request and nothing blocks on it. Read the last **inbound** line in `world.log` with nothing after it. |

### Then, without relaunching, two more free readings

* **Relog.** The job must still be 100 on the character-select list and on the next
  `SetField` — that exercises `character_stat_block`'s `job` at read #11 (`+0x33`) and its
  SP fork on the *new* job. If character select shows the character but entering the world
  hangs, the SP encoding is the suspect and `jobs::sp_encoding_changes` says it should not
  be — which would make it a real finding.
* **`!job 0`.** The fanfare must **not** play (`142d55beb test ax,ax / je`). A silent revert
  is a second, independent confirmation that the gate this analysis read is the gate that
  runs.

**Do not combine this with anything else**, and in particular do not add the instructor
dialogue in the same run: two variables have already produced one unexplained crash here.

---

## 12. What I could not settle

* **The two `u16`s in `0x02D1` effect 14** (§4.4). `FUN_1429dc320` has no `.pdata` entry and
  was not disassembled. It does not block anything, because the recommendation is not to send
  that packet at all — but a remote-player fanfare would need it.
* **Whether clicking an instructor produces a `0x00F2` at all** (§5.2). Two client-side
  branches — `FUN_1401d3220(tid)` and `template->[0x178]` at `141e3c72b` — can end the click
  with no packet, and neither was disassembled. The prediction is `0x00F2` site C and the
  reasoning is [D], but it is a prediction. **This is why the §11 test does not depend on it.**
* **Whether `subJob` should be `0`.** Every character this server has sends `0` there today
  and the client accepts it; nothing was read that says what a non-zero value means.
* **What `SpPool::job_level` contains** — unchanged from `stats.rs`'s own note, and it is
  the reason §9.2 recommends deferring the SP decision to the moment SP is first awarded.
* **The advancement dialogue.** Not recoverable: the instructors have four to six idle lines
  each and nothing else (§6.1). Whatever is written will be ours.
