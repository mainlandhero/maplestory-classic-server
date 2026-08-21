# Where `q1002s` lives — and why Roger says "Hey, nice weather, isn't it?"

**Written 2026-08-21. No Ghidra** (another agent held the project lock). Instruments: the
repo's `target/release/wz-dump`, a full-tree enumeration script, a byte/UTF-16 scan of
`client-patched/MapleStory.exe`, `world.log` from the 06:0x run, and the v214 tree at
`C:\Users\user\Desktop\ModernMapleSource` for shape only.

Labels: **[L]** read off a listing, a capture or the client's own data; **[D]** derived from
two or more [L] facts; **[I]** inferred — anything from the v214 tree is a candidate only
(it is a different game version and scored 1 of 8 against a held-out control).

---

## 0. Answer up front

| question | answer |
|---|---|
| Where is the body of `q1002s`? | **Nowhere in the client.** Enumerated, not searched: 205 archives, **10 021 images**, 0 decode errors. `q1002s` and `q1002e` occur **exactly twice in the whole tree**, both as the *names* in `1002.img`. **[L]** |
| Is there a `Script.wz`? | **Yes, and it is empty.** `Data/Etc/Script/Script_000.wz` is **63 bytes** — a `PKG1` header with **zero entries**. `Script.ini` says `LastWzIndex\|0`. **[L]** |
| Loose `.js` / `.lua` / `.txt` script trees? | **None.** Every one of the 435 files under `client-patched/` is `.wz .ini .dll .jpg .log .exe .pak .aes .bin .bat .sys .dat .orig .stub`. **[L]** |
| Is the body in the executable? | **No.** `q1002s` / `q1002e` are absent in both ASCII and UTF-16LE across all 76 702 712 bytes. The *field names* `startscript` / `endscript` / `failscript` are present, once each, as UTF-16 — the WZ property names the client reads. **[L]** |
| So where were they? | **Server-side, in the original game.** The client's own GM command guide ships `loadscript <file>`, `scriptrun <name>`, `runlua`, `loadlua` and `loadquest` — commands whose only purpose is to make the *server* re-read script files. **[L]** the strings; **[D]** the reading. |
| What does the client need on the wire to run a scripted quest start? | **Nothing it is not already being sent.** `0x0151` action 4 is answered with the same `0x055B` / `0x00F3` loop a Say-tree quest uses. The mechanism is already proven live in this very run — the client *drew* the server's answer. Only the text was wrong. **[L]** |
| So how big is the fix? | **Data, not protocol.** Nine TSV rows in `data/quest-scripts.txt` and a ~15-line overlay loader. The session state machine needs **no change**. **[D]**, desk-checked in §7. |
| How many quests are affected? | **12** of 322 have a `startscript`, and **all 12** have no `Say."0"`. 15 have an `endscript`, 4 a `failscript`. **[L]** |

---

## 1. The measured failure, end to end

`world.log`, the 06:0x run, lines 248–252 **[L]**:

```
06:10:04.854  <- 0x0151  13 byte body  04 ea030000 03000000 44ff1301
                         action 4, quest 0x3ea = 1002, npc template 3, x=-188 y=275
06:10:04.854  -> 0x055B  ScriptMessage Say from NPC template 3, line 1 of 1 on path ""
06:10:06.053  <- 0x00F3  40 byte body  00000000 00 00000000 1c00
                         "Hey, nice weather, isn't it?" 01
```

Three things are worth reading off that, because together they say the protocol is fine:

* `path ""` and `line 1 of 1` — the server had **no quest lines at all** and fell through to
  the NPC's own line.
* the 28-byte string in the `0x00F3` is `String.wz/Npc.img/3/d0` **verbatim**, which is what
  `Session::npc_line` returns. **[L]** So the owner's screen and the log agree on the same byte.
* the client **echoed it back**, i.e. it accepted, rendered and answered a `0x055B` that the
  server sent in reply to an action-4 request. **The scripted-start round trip already
  works.** Nothing about `action 4` needs new machinery. **[L]**

### 1.1 The exact line of code, and why it does what it does

`crates/world/src/session/npc.rs:84`, `on_quest_request`'s path chooser. For action 4:
`state = "0"`, `accepted = false`, `completing = false`. Quest 1002's `say` map, built by
`config::load_quests` from `gm-handbook/questlines.txt`, has **one key**: `"1.stop.item"`.

```
arm 1  accepted && say has "0.yes"     -> accepted is false
arm 2  say has "0"                     -> no
arm 3  completing                      -> false
arm 4  say has "0"                     -> no
_                                      -> None
```

`path = None` ⇒ `quest_id = None` ⇒ `say_lines` takes the `None` branch ⇒
`npc_line(3)` ⇒ `d0`. **[D]**, and §7's simulation reproduces the log line exactly from this
chain, which is the control for the prediction that follows.

---

## 2. The enumeration, with every negative named

`CLAUDE.md`'s rule is enumerate before you filter, so nothing here was filtered. Every `.wz`
under `client-patched/Data` was opened with `wz-dump tree`, every image it named was decoded
with `wz-dump cat`, and the decoded JSON was matched against
`q1002s|q1002e|startscript|endscript|failscript|script`.

```
ARCHIVES 205        TOTAL_IMAGES 10021       ERRORS 0        HIT_IMAGES 543
```

`_Canvas` and the two 90 MB `Sound` archives were **not** skipped — a negative that came from
an archive list would be a property of the list.

**Instrument controls, run before any negative below was believed:**

* the token regex found 14 429 `scriptPortal`, 533 `script` and 375 `fieldScript` nodes, so
  it can see a `script`-shaped key;
* it found the two `1002.img` rows, so it can see the exact string being looked for;
* `find` over `client-patched/` returns 435 files and a full extension histogram, so the
  "no `.lua`" negative is not an empty search path;
* the exe scan found `QuestInfo` (ASCII, 1 hit) and `startscript` (UTF-16, 1 hit), so both
  encodings speak.

### 2.1 Every candidate home, by name, including the absent ones

| candidate | what is actually there | verdict |
|---|---|---|
| **`Data/Etc/Script/Script.wz`** | exists; `Script_000.wz` is **63 bytes**, `PKG1` header, **0 entries**. `Script.ini` = `LastWzIndex\|0` | **empty archive [L]** |
| **`Data/Etc/Etc.wz`** | 67 images. `ScriptInfo.img` is present and decodes to `{}` (13 bytes, the empty-image size). `QuestProgressManualMsg.img`, `NpcNoticeBoard.img`, `RecommendSkill.img` are also `{}`. `NotShowQuestList.img`, `ScanBlockQuest.img`, `MedalQuestCategory.img` are quest *metadata*, no text | **no script bodies [L]** |
| **`Data/Quest/Quest.wz`** | 8 images: `ChangeableQExpTable`, `Exclusive`, `PQuest` (13 B, empty), `QuestCategory`, `QuestDestination`, `QuestExpByLevel`, `QuestPerformByDay`, `RecurringQuestGroup`. **There is no `Say.img` / `Act.img` / `Check.img` / `QuestInfo.img` sibling** — this client splits per quest | **no script bodies [L]** |
| **`Data/Quest/QuestData/`** | 322 images, one per quest, each holding that quest's `QuestInfo` / `Check` / `Act` / `Say`. This is where `startscript` lives, and it holds the **name only** | **names only [L]** |
| **`Data/String/String.wz`** | 24 images. **No `Quest.img`** (this client keeps quest text in `QuestData`), no `CommandGuide.img` | **no script bodies [L]** |
| **`Data/Npc/Npc.wz`** | 266 images. Roger, `0000003.img`, has `info/speak = {n0,n1,n2}` and **no `script` node at all** | **no script bodies [L]** |
| **`Data/Mob/QuestCountGroup/`** | exists; `QuestCountGroup_000.wz` is 63 bytes, **0 entries** | **empty archive [L]** |
| **any `.img` whose name contains "script"** | **exactly one in 10 021**: `Etc.wz/ScriptInfo.img`, and it is `{}` | **[L]** |
| **loose `.js` / `.lua` / `.txt` under `client-patched/`** | **zero**. 435 files, extensions: 205 wz, 102 ini, 56 pak (Chromium overlay), 37 dll, 13 jpg, 6 log, 4 exe, 3 aes, 2 bin, 2 bat, 1 sys, 1 dat, 3 backup suffixes | **[L]** |
| **`Data/Etc/Language/es/…`** (the Spanish overlay) | a second, smaller quest tree — 265 images. `1002.img` there has translated `QuestInfo` and `Say.1.stop.item.0` and **again no `Say."0"` and no script body** | **[L]** |
| **`client-patched/MapleStory.exe`** | `q1002s`/`q1002e` absent in ASCII and UTF-16LE. `startscript`/`endscript`/`failscript` present as UTF-16, adjacent to `selfComplete`, `autoComplete`, `scenarioQuest`, `timeLimit2` — i.e. inside the client's `Quest.wz` property-name table | **names only [L]** |

### 2.2 What the whole tree *does* contain that is script-shaped

Every `script`-ish node in all 10 021 images is a **name**, never a body:

| key | count | example |
|---|---|---|
| `scriptPortal` | 14 429 | `Map.wz` — `"scriptPortal": "talkToMai"` |
| `script` | 533 | `Item.wz/Cash/0518.img` — `"script": "cash_5180000"` |
| `fieldScript` | 375 | `Map0_000.wz/000000001.img` — mostly `""` |
| `endscript` | 15 | `QuestData/1002.img` — `"q1002e"` |
| `startscript` | 12 | `QuestData/1002.img` — `"q1002s"` |
| `failscript` | 4 | `QuestData/20002.img` — `"q20002f"` |

**That is the shape of the answer.** Portals, items, fields and quests all name scripts the
same way, and the client ships **none** of the 15 368 bodies those nodes name. A data file
that names fifteen thousand scripts and defines zero of them is not a data file with a hole
in it; it is a client talking to a server that owns the scripts.

### 2.3 The client's own GM command guide says so out loud

`Data/Etc/Language/es/String/String_000.wz` → `CommandGuide.img` — an image with **no
English counterpart** in `Data/String/String.wz`, which is why it had not been read before.
Among the admin commands it documents **[L]**:

```
"loadscript 파일명"   : "Carga los archivos de script indicados."      (loads the named script files)
"scriptrun 스크립트명" : "Ejecuta el script indicado."                  (runs the named script)
"runlua"             : "Ejecuta el script de Lua si hay un campo de script."
"loadlua"            : "Vuelve a ejecutar el script de Lua si hay un campo de script."
"loadquest"          : "Lee el archivo de configuración de misiones."  (reads the quest config file)
```

A GM command is typed in the client and acted on by the server. `loadscript` and `loadquest`
exist **because the script files and the quest configuration are files the server holds** and
occasionally has to re-read. **[L]** the strings; **[D]** the conclusion — but it is the same
conclusion the enumeration reaches independently, and the two do not share a blind spot: one
is an exhaustive decode of the data, the other is a sentence in it.

`runlua` also names the language. That is **[L]** for "Lua exists somewhere in this game's
server tooling" and **[I]** for anything about `q1002s` specifically — the naming convention
`q<id>s` is shared with the v214 Python tree (§5), so the language is not settled and does
not matter: MapleCW is not going to embed an interpreter.

### 2.4 What was *not* checked, and why that is safe

The `_Canvas` archives were decoded with the same `cat` as everything else, but their images
are bitmaps: a body hidden there as raw canvas bytes would not show up as a JSON string.
Naming the blind spot as `CLAUDE.md` requires: **a script body stored as an image's pixel
payload, or as a `_raw`/binary property, would be invisible to a token scan of the decoded
JSON.** Two things make that not worth chasing. First, `Script.wz` exists and is *empty* —
the client author made a place for scripts and shipped nothing in it, which is not what
someone hiding them in canvases does. Second, `Etc.wz/ScriptInfo.img` — the one image in the
whole client whose name says "script" — is `{}`, the same signature. Three independent empty
containers is a positive finding, not an absence.

---

## 3. `Quest.wz/QuestData/1002.img` in full, decoded

```json
{
  "QuestInfo": { "name": "Roger's Apple", "area": 1,
    "0": "Talk to #b#p3##k.",
    "1": "Press the hotkey #bI#k to open your inventory, navigate to the #bUse#k tab, and
          double-click #b#t2010000##k to consume it. Then, talk to #b#p3##k.",
    "2": "You learned how to use items! This will make life much easier!" },
  "Check": {
    "0": { "npc": 3, "lvmin": 1, "startscript": "q1002s", "job": { "0": 0 } },
    "1": { "npc": 3, "endscript": "q1002e", "item": { "0": { "id": 2010000 } } } },
  "Act":  { "0": {}, "1": {} },
  "Say":  { "0": {},
            "1": { "stop": { "item": { "0": "Eat the #r#t2010000##k I gave you by opening
                   your inventory and clicking the #bUse tab#k. …" } } } }
}
```

Four things in there decide the whole design **[L]**:

1. **`Say."0"` is present and empty.** Not missing — *empty*. The slot exists and the script
   was to fill it.
2. **`Act."0"` and `Act."1"` are both empty too.** So the apple is **not** given by
   `Act.0.item`, and the reward is **not** in `Act.1.exp`. A script quest's *effects* are the
   script's as much as its text is. Any design that only supplies dialogue leaves the player
   unable to finish, because `Check.1` wants item 2010000 and nothing would ever hand it over.
3. **`Check.1.item.0` has an `id` and no `count`.** 214 other quests in this client do carry
   `Check.1.item.0.count`, so the absence is data, not a dumper artefact. `tools/dump_quests.py`
   emits **zero** rows for 1002 in `gm-handbook/questreq.txt` as a result. The natural reading
   — count 0 means "you must **not** hold it any more", i.e. you must have *eaten* the apple —
   is **[I]**, and it is why the `stop` line is *"Eat the apple I gave you"*.
4. **The `stop` node is the failure text**, and §6 is about the fact that nothing routes to it.

`Data/String/String.wz/Consume.img/2010000` **[L]**: name *"Roger's Apple"*, desc *"A ripe,
red apple. Restores 30 HP."* And `Npc.img/3` **[L]**: `d0` = *"Hey, nice weather, isn't it?"*
— the sentence the owner saw — plus `n0`/`n1`/`n2` (an Item-Inventory tutorial), `f0`–`f2`,
`w0`–`w2`, and `d1` = *"Was my information helpful? Farewell, my dear Mapler!"*.

**Roger's own strings are enough to write most of `q1002s` without inventing anything.** That
is what `data/quest-scripts.txt` does, and it says line by line which words are their and which
are ours.

---

## 4. The 12 script quests, and the 15 end scripts

From `gm-handbook/questlines.txt` **[L]**:

| quest | start | end | fail | has `Say."0"` | name |
|---|---|---|---|---|---|
| **1002** | `q1002s` | `q1002e` | — | **no** | **Roger's Apple** |
| 20002 | `q20002s` | `q20002e` | `q20002f` | no | Test of Qualification |
| 20102 | `q20102s` | `q20102e` | `q20102f` | no | Test of Qualification |
| 20202 | `q20202s` | `q20202e` | `q20202f` | no | Test of Qualification |
| 20302 | `q20302s` | `q20302e` | `q20302f` | no | Test of Qualification |
| 500000 | `q500000s` | `q500000e` | — | no | [Event] A Strange and Familiar Wood |
| 500001 | `q500001s` | `q500001e` | — | no | [Event] Beyond the Unknown |
| 500002 | `q500002s` | `q500002e` | — | no | [Event] The One Who Sealed the Darkness |
| 500005 | `q500005s` | `q500005e` | — | no | [Event] Gathering Leaves |
| 500006 | `q500006s` | — | — | no | [Event] Gift for the New Journey |
| 500008 | `q500008s` | `q500008e` | — | no | [Event] A New Adventure Awaits |
| 500009 | `q500009s` | `q500009e` | — | no | [Event] Growing Leaf |
| 20003 / 20103 / 20203 / 20303 | — | `q<id>e` | — | **yes** | Proof of Qualification |

**Every quest with a `startscript` has no `Say."0"`, and every quest without one has a
`Say."0"` — with a single exception, quest 80000, which has neither.** So the two kinds
partition cleanly and the client's fork is exactly what `script.rs` documents: action **1**
when there is no start script, action **4** when there is. **[L]**

The last four rows are the interesting shape: `20003` etc. have a normal client-side opening
(`Say."0"` exists → action 1) and a *scripted* close (action 5). Those need only the `Say."1"`
half authored.

Roger is the **only** NPC on `Check.0.npc = 3` / `Check.1.npc = 3`, so quest 1002 is their
entire repertoire. Fix it and the tutorial's second half works. **[L]**

---

## 5. What a quest script has to be able to do — shape only, from v214 **[I]**

`C:\Users\user\Desktop\ModernMapleSource\v214 src\data\scripts\quest\` holds **2 089** files
named `q<questId>s.py` and `q<questId>e.py`. **The naming convention is identical** — same
`q`, same id, same `s`/`e`, same `f` for fail. That is the only thing this tree is being used
for, and it is still **[I]**: it is a different game version and there is no `q1002s.py` in it.

The vocabulary a start script actually uses, across the ones read:

```python
sm.sendNext("…")          # a Say box with a Next button
sm.sendSayOkay("…")       # a Say box with OK
res = sm.sendAskYesNo("…") # the Accept / Decline box
sm.giveItem(2000068, 1000)
sm.startQuest(parentID)
sm.completeQuest(parentID)
```

Every one of those maps onto something MapleCW already has **[D]**:

| v214 call **[I]** | MapleCW today |
|---|---|
| `sm.sendNext` | `net::script::npc_say(t, text, prev=false, next=true)` |
| `sm.sendSayOkay` | `net::script::npc_say(t, text, false, next=false)` |
| `sm.sendAskYesNo` | `net::script::npc_ask(t, text, quest_flavoured=true)` → type `0x10` |
| `sm.giveItem` | `Session::grant_quest_start_items`, driven by `Act.0.item` |
| `sm.startQuest` | `Session::record_quest_start` |
| `sm.completeQuest` | `Session::record_quest_complete` |

**There is nothing here MapleCW cannot already say.** The gap is not capability, it is that
no rows exist to drive it. That is the whole finding.

---

## 6. `0x0151` **action 5** — the end script, and action 6, the one that is wrong

### 6.1 Action 5, `COMPLETE_SCRIPT`

From `crates/net/src/script.rs`'s listing read of the builder `FUN_141f0e4c0` **[L]**:

```
141f0e8cc   the 0x0151 site        13-byte body
141f0e8de   mov dl, 5              the tag
            u8 tag, u32 questId, u32 npcTemplateId, u16 charX, u16 charY
```

Identical shape to action 4 — same 13 bytes, same fields, only the tag differs. The gate is
the mirror of action 4's: **quest state is in-progress *and* the quest has an `endscript`**.
For any quest with an `endscript`, the client will send **5 and never 2**, exactly as it
sends **4 and never 1** for a quest with a `startscript`. **[L]** the gate as documented on
`QUEST_ACTION_COMPLETE_SCRIPT`; **[D]** the mutual exclusion.

**What the client does with it: nothing but send it and wait.** There is no local dialogue,
no local state change, no timeout. The completion conversation, the reward, the journal row
and the `Act."1"` effects are all the server's — the same machinery, at the other end. In
MapleCW's current code that is already true: `on_quest_request` maps action 5 to `state = "1"`
and calls `record_quest_complete`, which pays `Act.1.exp` and applies `Act.1.item`. It fires
today; it simply has nothing to say, because `Say."1"` is empty. **[D]**

So closing quest 1002 costs exactly two more rows: a `Say 1.0` and an `Act 1.exp`. Both are
in `data/quest-scripts.txt`.

### 6.2 Action 6, `REQUIREMENT_FAILED` — a real bug, and it is **not** fixed here

Tag 6 goes out when `FUN_140711d70(quest)` returns non-zero — a requirement check failed
**[L]**. `on_quest_request` sends every unrecognised action down the `_ => "0"` arm, so an
action 6 is answered with **the quest's opening conversation**.

That is the same class of bug as the one already written up in `npc.rs` — *"Falling back to
the start of a quest when asked to finish it is never right"* — and quest 1002 is exactly
where it would show: the player who has not eaten the apple gets Roger's pitch again instead
of `Say."1".stop.item.0`, which the client ships **for precisely this** and which
`questlines.txt` already carries.

**This is stated as a hazard, not as a measurement.** No capture in `previous-runs/` or
`research/fixtures/` contains a tag-6 `0x0151`; the reasoning is [D] from the builder table.
The fix is not in this change because it needs the quest's stored state to decide whether
the failing check was `Check.0` or `Check.1`, and picking the wrong one is how the completion
fell back to the opening the first time. Recorded loudly rather than half-done.

---

## 7. The design: an authored overlay, and the desk-check that predicts it

### 7.1 Why the format is `questlines.txt`'s format

A script quest needs: opening lines, an accept branch, a decline branch, an item to give, a
closing line, an experience payout. **The WZ schema already expresses all six**, and
`world::config::load_quests` already parses it. So `data/quest-scripts.txt` is the same
four-column TSV, and the only new code is a loader that applies rows **onto** an existing
quest map instead of building a fresh one.

That is genuinely the smallest thing that works. It also has a property worth having: the
authored rows are in the same shape as the extracted ones, so a mistake in them is visible
by diffing against `gm-handbook/questlines.txt` rather than by reading two languages.

The nine rows for 1002 are in `data/quest-scripts.txt`, each with its provenance —
five of the six text lines are Roger's own words from `String.wz/Npc.img/3`.

### 7.2 The desk-check

`on_quest_request` → `say_line` → `on_script_reply` was reimplemented line for line in
Python and run over the real `questlines.txt`, once alone and once with the overlay.

**Control first.** With the overlay absent, the simulation predicts:

```
action 4 -> path None, quest_id None, 1 line
   0x055B  Say next=0  "<Roger's d0>"
```

`world.log` recorded `ScriptMessage Say from NPC template 3, line 1 of 1 on path ""`. Same
path, same line count, same button state. **The simulation reproduces the measured failure**,
which is what licenses the prediction below. **[D]**

With the overlay applied:

```
action 4 -> path "0", quest_id 1002, 2 lines
   0x055B  Say next=1       "You'll die when your HP reaches 0, …"
   0x055B  YES/NO (0x10)    "Want to see how that works? I'll give you one of my apples…"
   [Accept] -> accept_quest -> record_quest_start(1002)
              + grant_quest_start_items -> item 2010000 x1
   0x055B  Say next=1       "Open your Item Inventory to find your Equip, Use, …"
   0x055B  Say next=0       "Double-click on an item in your Item Inventory and …"

action 5 -> path "1", quest_id 1002, 1 line
   record_quest_complete(1002) -> exp 3
   0x055B  Say next=0       "Was my information helpful? Farewell, my dear Mapler!"
```

Note what did **not** have to change: `on_quest_request`'s arm 2 (`say.contains_key("0")`)
picks up the overlay on its own; `say_line`'s `branches` test turns the last opening line
into a type-`0x10` Accept box on its own because `0.yes` now exists; `on_script_reply`'s
yes-branch calls `accept_quest` on its own. **The session state machine is untouched.** [D]

### 7.3 Why the Accept lands in `on_script_reply`, not in `on_quest_request`

Worth writing down because it is the opposite of the Say-tree case and it is the thing most
likely to be "fixed" wrongly later. For a Say-tree quest the client draws its own opening and
its own Accept, and the press arrives as `0x0151` **action 1**, which `on_quest_request`
records. For a **script** quest the client draws nothing: action 4 arrives *before* any
dialogue, `accepted` is false, and the Accept the player presses is on a box **the server
sent**, so it comes back as `0x00F3` with `message_type = 0x10` and `action = 1`. It is
`on_script_reply`'s `awaiting_yes_no` branch that must call `record_quest_start` — and it
already does. **[D]** from the two gates in `script.rs` plus `npc.rs`'s existing flow.

Corollary: **`QUEST_ACTION_START` will never arrive for quest 1002**, so any future code that
records a start only on action 1 will silently never start a script quest. **[D]**

---

## 8. WIRE IT LIKE THIS

**Nothing below is wired.** `data/quest-scripts.txt` exists and parses; no Rust reads it yet.
That is `CLAUDE.md`'s "built is not wired" state and it is being declared, not hidden.
`crates/world/src/session/` belongs to the coordinator and was not touched.

### 8.1 The packets, in order

For quest 1002, NPC template **3**, using the shapes in `research/script-reply.md`:

```
1.  client -> 0x0151   04 ea030000 03000000 <x:u16> <y:u16>          13 bytes
                       action 4 (OPENING_SCRIPT), quest 1002, npc template 3

2.  server -> 0x055B   Say, line 0, prev=0, next=1, speaker 3        (npc_say)
3.  client -> 0x00F3   handle 0, msgType 00, echo 0, <text>, action 01

4.  server -> 0x055B   msgType 0x10, speaker 3, flags 0              (npc_ask, quest-flavoured)
                       draws BtQYes / BtQNo. THIS is the Accept button.
5.  client -> 0x00F3   00000000 10 01                                6 bytes -> YES
                       (00 = No -> send Say."0".no ; FF = closed -> send nothing)

6.  server -> 0x0089   quest::quest_accepted(1002)                   the journal row
    server -> 0x0070   inventory_added(Use, slot, Roger's Apple)     Act.0.item
    server -> 0x0089   message::item_gained(2010000, 1)
    server -> 0x055B   Say."0".yes line 0, next=1
7.  client -> 0x00F3   … action 01
8.  server -> 0x055B   Say."0".yes line 1, next=0                    last line
9.  client -> 0x00F3   … action 01                                   -> conversation ends,
                                                                        server sends nothing

--- player eats the apple, then clicks Roger again ---

10. client -> 0x0151   05 ea030000 03000000 <x> <y>                  13 bytes
                       action 5 (COMPLETE_SCRIPT). NOT action 2 - 1002 has an endscript.
11. server -> 0x0089   quest::quest_completed(1002, filetime)
    server -> exp      award_experience(3)                            Act.1.exp
    server -> 0x055B   Say."1" line 0, next=0
12. client -> 0x00F3   … action 01                                   -> done
```

Opcode names are `net::quest::MESSAGE`, `net::inventory::INVENTORY_OPERATION`,
`net::message::MESSAGE`, `net::script::SCRIPT_MESSAGE` — all already in use for quests 1000
and 1001, all already producing the right bytes. Steps 2–9 and 11–12 are `say_line` and
`on_script_reply` **unchanged**.

Three rules from `research/script-reply.md` that apply unchanged and are cheap to break:

* **one box at a time** — never send the next `0x055B` before its `0x00F3` arrives;
* **never send `0x055B` with or just before a `SetField`** — field entry resets the script
  manager and eats the dialog;
* **always answer** — but note that *ending* a conversation by sending nothing is the correct
  and safe thing, because `0x00F3` is not one of the 37 setters of `player->[0x2330]`.

### 8.2 The code, which is three edits and none of them in `session/`

1. **`crates/world/src/config.rs`** — add an overlay entry point beside `load_quests`:

   ```rust
   /// Apply an authored overlay (`data/quest-scripts.txt`) onto quests already loaded from
   /// the generated `questlines.txt`. Same TSV, same parser; the only difference is that
   /// rows are merged into an existing map instead of building a new one.
   pub fn overlay_quests(quests: &mut HashMap<u32, Quest>, path: &std::path::Path)
   ```

   The cheapest correct implementation is to factor the body of `load_quests` into
   `fn read_quest_rows(text: &str, out: &mut HashMap<u32, Quest>)` and call it twice.
   **Do not** re-derive the `Say` line-index stitching by hand — the `rsplit_once` /
   `BTreeMap<usize, String>` ordering in `load_quests` is what makes a ten-line node come out
   in index order rather than string order, and re-implementing it is how `0.10` ends up
   before `0.2`.

2. **`crates/world/src/bin/world_server.rs`** — a `--quest-scripts PATH` flag defaulting to
   `data/quest-scripts.txt`, applied right after `quests_path` is loaded. Same handling as
   `data/shops.txt`: authored source, cannot be regenerated, absent file is not fatal.

3. **`crates/world/src/session/` — nothing.** Confirmed by the §7.2 desk-check. If a change
   turns out to be needed there, it is the coordinator's, and it is a bug in this analysis.

### 8.3 What to watch for on the run, and what each outcome means

One variant, one launch. Click **Roger** on Maple Island with a fresh character.

| what the owner sees | what it means |
|---|---|
| *"You'll die when your HP reaches 0…"* with a **Next** button | the overlay loaded and `path "0"` was chosen. The whole chain is live. |
| the second box has **Accept / Decline** (not OK) | `say_line` turned the last line into type `0x10`. This is the thing that has failed before with a plain Say drawing OK. |
| **Roger's Apple appears in the Use tab** after Accept | `Act.0.item` overlay reached `grant_quest_start_items`. |
| **the quest appears in the quest journal** | `record_quest_start` fired from `on_script_reply`, i.e. §7.3 is right. |
| *"Hey, nice weather, isn't it?"* still | the overlay did **not** load — check the `--quest-scripts` path, not the dialogue code. This exact sentence is the fall-through signature. |
| the opening plays but there is no Accept box | `0.yes` did not parse; check the tab characters in `data/quest-scripts.txt`. |
| the quest starts but Roger has nothing to say on the second click | action **5** reached a missing `Say."1"` — or the client sent **6**, not 5, because the apple is still in the bag (§6.2). `world.log`'s tag byte settles which in one line. |

`world.log` answers all six without a second launch: the tag byte of each `0x0151` and the
`on path "…"` label of each `0x055B` say exactly which arm ran.

---

## 9. What I could not settle

* **Whether `Check.1.item.0` with no `count` means "must not hold"** (§3, item 3). It is the
  only reading that makes the `stop` line sensible, and 214 other quests do carry a count —
  but it is **[I]**, and `tools/dump_quests.py` emits no `questreq.txt` row for 1002, so the
  server has no gate on it either way. Completion is currently ungated for this quest.
* **Whether the client ever sends action 6 for quest 1002** (§6.2). No capture contains a
  tag-6 request. The hazard is [D] from the builder table, and the fix is deliberately not
  attempted here.
* **The original wording of `q1002s`.** Not recoverable from anything on this machine. What
  `data/quest-scripts.txt` ships is five of Roger's own six lines plus two sentences of ours,
  each labelled. The reward, `Act.1.exp = 3`, is **ours and unsourced** — the client carries
  no reward for 1002 at all.
* **The other 11 script quests.** Only 1002 is authored. The four `Proof of Qualification`
  quests (20003/20103/20203/20303) are the cheapest next ones: they have a client-side
  opening already and need only a `Say."1"`.
* **`fieldScript`, `scriptPortal` and item `script`** — 15 337 more named-and-undefined
  scripts, same situation, not chased. `scriptPortal` is the one that will matter next: a
  portal with a script name is a portal the server has to answer for.

---

## 10. Instrument notes

* **The sweep script was piped in (`python - < script.py`) exactly as `CLAUDE.md` says**, so
  `sys.path[0]` is empty and the scratchpad's stale copies of `reads.py`, `dis.py` and friends
  cannot shadow `tools/`. It also meant `__file__` was undefined and the JSON dump at the end
  of the script raised — after the results had printed. Harmless here, and the re-dump of the
  543 hit images was cheap; but the lesson is that a piped script has no `__file__` and must
  take its output path as an argument.
* **`grep -a` on the exe under-reports by design.** It finds narrow ASCII only, and this
  binary keeps its WZ property names as UTF-16LE. `startscript` returns **0** hits as ASCII
  and **1** as UTF-16. A search for `q1002s` that used only the ASCII form would have come
  back clean, confident and uninformative. Both encodings were run, each with its own control.
* **`Etc/Language/es/String/String_000.wz/CommandGuide.img` has no English counterpart.** The
  single most useful sentence in this investigation is in the Spanish overlay only. A sweep
  scoped to `Data/String/` would have missed it; enumerating all 205 archives found it.
* **The empty archives are 63 bytes and decode without error**, so `wz-dump tree` returns
  success and an empty list. `imgs=0` is a real answer, not a failure — but a script that
  only checked the return code would treat `Script.wz` as fine and move on.
* **Positive control before every negative.** The token regex was shown to find 14 429
  `scriptPortal` nodes and both `1002.img` rows before "not found anywhere else" was written
  down; `find` was shown to return 435 files before "no `.lua`" was; the exe scan was shown to
  find `QuestInfo` and `startscript` before "no `q1002s`" was.
* **The §7.2 simulation was run against the *pre-fix* data first** and had to reproduce
  `world.log`'s `path "" / line 1 of 1` before its post-fix prediction was allowed to count.
  A simulator that only ever runs on the fixed data cannot fail.
