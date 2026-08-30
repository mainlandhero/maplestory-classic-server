# Archive sweep, 2026-08-29 — closing open questions without a client run

Five questions were marked "needs a client run". **Three close from data already on disk,
one closes by running a smoke test that needs no client, and one stays open** — but the open
one now has a named experiment that costs ten seconds instead of a full launch.

Along the way the sweep found that **a repo-wide redaction commit silently defeated the
project's own de-duplication rule**, and that **the current test plan's step T5 will produce
a false negative if the owner runs it on the second account**. Both are in §5.

Labels: **[L]** measured · **[D]** derived · **[I]** inferred.

Instruments built for this sweep:

| | |
|---|---|
| `tools/archive_events.py` | every packet line in every archived log, keyed on `(timestamp, direction, opcode, body)`, with `--unredact` (see §5.1). Blind spots in its docstring. |
| `scratchpad/sweep-agent/pair_fixtures.py` | pairs each `research/fixtures/` log with its `previous-runs/` original under four progressively weaker keys |
| `scratchpad/sweep-agent/overlap.py` | connection-overlap detector with a built-in positive control |

Corpus: 432 log files across `previous-runs/`, `research/fixtures/` and the repo root.
**339 619 distinct packet events** from **593 585 raw log lines** [L]. The gap is what
`fixtures/` being copies costs, and it is the whole reason for §5.1.

---

## 1. `0x010E` for a `0203` item (return scrolls) — **HALF CLOSED**

### The opcode is live, and it is not dead code [L]

`0x010E` is in the archive **10 distinct times across 9 distinct runs**. Every one is
`<-` (client to server), 14 bytes, and every body parses on `net::useitem`'s layout:

```text
tick(u32)  slot(i16)  itemId(u32)  tail(u32)
f6224811   0100       90ab1e00     01000000    <- item 2010000, Roger's Apple, Use slot 1
307d4a11   0100       80841e00     01000000    <- item 2000000, Red Potion,    Use slot 1
f0a79e1f   0300       80841e00     01000000    <- item 2000000, Use slot 3
```

Every one of the ten carries **2000000 or 2010000** — the two `0200`/`0201` HP potions.
`0203` appears in none of them. [L]

### No archived run has ever had a `0203` item in a bag [L]

Every item id the server ever named in a log line, deduplicated, falls into 32 four-digit
classes: `1002 1040 1041 1050 1060 1061 1062 1072 1082 1302 1312 1322 1332 1372 1382 1402
1412 1432 1442 1452 2000 2010 2040 2041 2060 2061 4000 4010 4020 4031 5000 5150`.
**`2030` is not among them.** The only two `!item` grants in the whole archive are `1302000`
and `1002998`, both Equip.

Searched three ways, each with a positive control:

| search | result | control |
|---|---|---|
| little-endian hex `b0f91e00` / `b9f91e00` (2030000 / 2030009) in packet bodies | 0 files | `80841e00` (2000000) → 73 files [L] |
| server text `item 203…` | 0 lines | `item 200…` → present [L] |
| decimal `2030000` as text | matches only inside unrelated hex runs | — |

### So the honest sentence

**The `0x010E` path is live code and the client sends it for Use-tab consumables. Whether it
also fires for a `0203` return scroll has never been observed, because no archived run ever
contained one.** [L] That is a different sentence from "the path is dead", and T5's third
outcome ("nothing happens at all → the client never sent `0x010E`") is still the thing to
watch for — but it is now a *narrow* question about one item class, not about the opcode.

**A bonus that upgrades an `[I]`:** `useitem.rs` says the once-per-session latch is `[I] by
analogy`. `previous-runs/world-20260821-202046.log` carries **two** `0x010E` in one session,
at 00:12:30.370 and 00:15:07.169, and the first was answered with `0x007C StatChanged` 3 ms
after it arrived. So *within one capture*, answering a `0x010E` permits a later one [L].
The other half — that leaving one unanswered blocks the next — still rests on a different
session (2026-08-21 09:59, one `0x010E` while the owner says they tried more than once), and
CLAUDE.md's rule about not welding two sessions into one observation applies.

### Cheapest experiment that closes it

On the **GM account** (`maplecw` — see §5.3), one action, at any point in a run the owner is
already making:

> `!item 2030000 1`, then double-click it in the Use tab.

* a `0x010E` appears in `world.log` → the client builds it for `0203` and the whole return
  scroll feature is testable. Everything else in T5 follows.
* no `0x010E` → the double-click handler is gated on item class, and the return scroll needs
  a different inbound opcode. That is the finding, and it is worth one Ghidra pass on
  `FUN_142cc8ab0`'s single call site at `142cca557` — which nobody has decompiled yet
  (`research/` has no file covering it).

---

## 2. `-SessionTokens` runs, and the `0x0073` identity block — **CLOSED, and the standing count is wrong**

### The count on record is a redaction artefact [L]

A naive body census says **two distinct `0x0073` bodies**:

```text
58  <-  0x0073  050000000000 d843ae4c5617 b6ae9cd2 00000000764d00000000
15  <-  0x0073  050000000000 aabbccddeeff deadbeef 00000000764d00000000
```

The second is not a second machine. `aabbccddeeff` / `deadbeef` are the **placeholders commit
`4f6b448` substituted for the capturing machine's MAC and machine id**, throughout the working
tree and all 458 commits, before the repo was published. `research/fixtures/README.md` says so
in as many words: *"do not conclude anything from the fact that they are identical across
captures"*.

Proof rather than inference: `research/fixtures/amherst-1013-heap-corruption-login.log` and
`previous-runs/login-20260819-222802.log` are **byte-identical after `CRLF→LF` and the two
substitutions** — same run, same millisecond timestamps, same 95 lines. [L]

Folding the redaction back:

```text
67  <-  0x0073  050000000000 <MAC> <machineid> 00000000764d00000000     ONE body, 67 launches
```

**[L] One identity block, across 67 distinct launches** (57 archived login logs + the live
one + 9 fixture-only login logs whose originals have rolled out of `previous-runs/`). That is
a *stronger* constancy result than "two bodies", and it removes the only reason anyone had to
think the block varies.

### Was any archived run launched with `-SessionTokens`? — **not answerable from the logs, and here is why** [L]

* **No archived log records the client's command line at all.** `NXLDEBUG` appears in zero
  archived logs; so do `argv` and `command line`. The launcher's own banner
  (`session tokens (config +0x90): …`) is a `Write-Host` to the **PowerShell console**, which
  no file captures.
* Therefore the archive is *structurally incapable* of telling which runs passed the flag.
  This is a property of the search, not evidence of absence.

What *is* on record: STATUS.md §"MEASURED: a launch-argument token does NOT reach the server"
dates the one token run to **2026-08-18**, and its capture is
`research/fixtures/session-tokens-no-effect.log`.

**That run is outside the login-log population entirely.** The earliest archived `login.log`
is 2026-08-19 22:04; the token run predates the Rust login server's logging. Its capture is a
71-line **Python probe listener** log, not a `login.log`. So:

> **None of the 67 `0x0073` observations is from a session-token run.** The "identity block is
> constant" measurement and the "tokens do not reach the wire" measurement have **no runs in
> common**. [L]

### And the token negative is weaker than STATUS.md presents it

STATUS.md says *"No token text anywhere in the capture."* The capture available today
**truncates every body over 16 bytes**:

```text
<- #0   0x0070 version report   body=45B 02 64 00 00 00 00 00 00 00 00 00 00 00 02 00 00...
<- #28  0x0073                  body=26B 05 00 00 00 00 00 aa bb cc dd ee ff de ad be ef...
```

Sixteen bytes shown, the rest elided. A token sitting at byte 17+ of `0x0070`, `0x0071` or
anything else in that run **could not have appeared** in this file. [L]

The claim survives anyway for `0x0073` specifically — the body is 26 bytes and the known field
layout (`u32 mode | str "" | 6-byte MAC | 4-byte machine id | u32 0 | u32 0x4d76 | u16 0`)
accounts for all 26, leaving nowhere for six tokens [D] — and the independent static
instrument (`FUN_142c95f20` has no Xrefs, in a run where sibling accessors did return callers)
does not share that blind spot. But **"no token text anywhere in the capture" is not a
sentence this capture can support**, and if the other agent is weighing how strongly the
token route is closed, that is the caveat to carry.

### The concurrent session-args pass has this one field wrong

`research/client-session-args.md` (being written by another agent as I write this) records the
same census and annotates the second body:

```text
 72 distinct 0x0073 EVENTS
  2 distinct bodies
  x57  05000000 0000 d843ae4c5617b6ae9cd2...   <- this machine
  x15  05000000 0000 aabbccddeeffdeadbeef...   <- the smoke test's synthetic
```

**Those 15 are not smoke-test output.** They are real client packets from **15 real launches**,
sitting in `research/fixtures/*-login.log`, with the machine identifiers replaced by commit
`4f6b448`. Six of the 15 are redacted copies of runs that are *also* in `previous-runs/` under
the real MAC — the same packet, same millisecond, counted twice. The remaining nine are runs
whose originals have rolled out of the buffer.

So the honest line for that table is **one body across 67 launches**, and the sentence
downstream of it — that the field is per-machine — is unchanged in substance but rests on a
census that was double-counting. (Their 72 vs my 73 is the live `login.log` at the repo root,
which their sweep did not include.)

### For the agent working on sessions, in one line

> The `0x0073` block is **one body across 67 launches**, not two [L]; **none of those 67 runs
> passed session tokens** [L]; and the single run that did is a 2026-08-18 probe capture whose
> bodies are truncated at 16 bytes [L]. The negative is real for `0x0073` on length grounds
> [D], and untested for every other opcode in that run.

---

## 3. Two clients on one machine — **NEVER OBSERVED, and nobody has ever tried**

Four independent instruments, each with a positive control. All four say the same thing.

| instrument | result | positive control |
|---|---|---|
| overlapping connections on either server | **211 connections in 188 logs, 0 overlapping pairs** [L] | synthetic overlapping pair is reported; synthetic sequential pair is not (`--selftest`) |
| `install_once` lines per hook log | 1 in every hook log but one; that one is 6.5 min later and says `env=false marker=false -> standing down` [L] | the parser finds the 1s |
| `client-exit.log`'s `protection/nexon processes at start/exit` | **never more than one `MapleStory#`**, across 30 such lines [L] | **42 of the same lines list two `maplecw-world#` processes** — the line demonstrably prints duplicates when they exist |
| socket monitor's `client pid(s)` | one pid, both times it ran [L] | the tool loops over `$procs` and prints each |

The connection detector is deliberately conservative: a connection with no end line is treated
as open to end-of-file, so it can only *over*-report overlap. It reported none.

**But that is "never observed", not "refuses to start".** Two client processes where the second
dies before `connect()` would leave **no trace in any of these files** — the second instance
would never reach a server, never load a hook log line the archive keeps, and never be the pid
`exit-forensics.ps1` was told to watch. So the plan's `STILL UNKNOWN` is correct, and it is
unknown because **the experiment has never been performed**, not because it was performed and
came back empty.

### Cheapest experiment that closes it — no server, ten seconds

The question is only whether the *process* starts. It needs no server, no login, no hook.

1. Double-click `C:\MapleCW\client-patched\MapleStory.exe`. Wait for the
   splash.
2. Double-click it again.
3. Watch for **ten seconds**, then close both. (Ten, not forty: the `__fastfail` from the
   firewalled reachability check lands at ~37 s, and a single-instance guard fires in the
   first seconds.)

* two splashes / two windows → **the client runs twice**, T1 and T2 are live, and the two-client
  plan is unblocked.
* the second instance vanishes, or a box says it is already running → **the answer is two
  machines**, and T1/T2 come off the plan until there is a second machine. Say which of the two
  it was, because "silently exits" and "says so" point at different guards.

This costs no elevated launch and no server start, so it can be done before deciding what the
next run is even for.

---

## 4. The six `channel_smoke.py --set-field-probe` failures — **CLOSED. All ten are stale expectations. Nothing is a server regression.**

Run today, no client: **87 PASS / 7 FAIL**. Two root causes, and the discriminating experiment
for each was free.

### 4.1 Six of the seven are one line: the smoke's account is not a GM

`session/gm.rs`, changed 2026-08-29 at the owner's request, gates **every** `!` command on
`accounts.is_gm`, and a refused command is **said out loud as ordinary chat** rather than
answered with a notice. `is_gm` defaults to 0 and `plant_character_and_migration` never sets
it, so all three `!` lines in the drive come back as `0x0231 UserChat` and nothing happens.

Everything downstream follows from that one fact:

| failure | actual cause |
|---|---|
| `map 40 sends 30 mobs` → 0 | `!map 40` never ran, so the character never left map 1 |
| `three are SetField` → 2 | the third SetField is the one `!map 40` would have sent |
| `refused !map explains itself` → 0 refusals among 0 notices | no `0x00BB` at all — no command ran |
| `ordinary chat is said back to the speaker` → 3 UserChat | all three lines echoed, GM commands included |
| `the text survives the round trip` → `b'!map 40'` | it reads `spoken[0]`, which is now the first `!` line |
| `the probe answered every request` → 18 replies | expected ≥ 40; the 30 mobs are the missing ones |
| `no-quest click answered as the TEMPLATE` → speaker 3 | the click landed on map 10 instead of map 40 |

**The experiment.** One added line — `UPDATE accounts SET is_gm = 1` after the account lookup
in `plant_character_and_migration` — and nothing else:

```text
        PASS   FAIL
before    87      7
after     98      3     <- all seven cleared
```

with the previously-failing checks now reading `map 40`, `3` SetFields, `1 refusals among 2
notices`, `b'Hello'`, `79 replies`, and `speaker 8` — the exact value the test's own comment
predicts. [L] That is as decisive as this gets.

Note what the smoke's *other* check did while all this was wrong: **`four are NpcEnterField`
passed** — because it counts. The four were map 1's two NPCs sent **twice**, byte-identical.
A count-only check passing beside a content failure is precisely the shape CLAUDE.md's
quest-fanfare section warns about, and here it is again.

### 4.2 The remaining three appear only once mobs exist, and they are pinned to a body length that changed

With the GM flag set, three new failures show up in code that could never run before:

```text
FAIL  a mob body is 137 bytes - 194 bytes
FAIL  no mob is sent with move action 0 - that is the crash - byte 35 values [0]
FAIL  no mob is sent with hp = 0 - that is a mob at 0 percent - []
```

All three are one stale constant. `0x03C6 MobEnterField` grew from **137 to 194 bytes**
between 2026-08-28 14:29 and 2026-08-29 00:09, when the mob's temp-stat mask stopped being
all zeros (the mob-attack work). Measured over the archive, event-deduplicated:

```text
body length   distinct MobEnterField events
        137   1731      every run up to and including 2026-08-28 14:29
        194    188      2026-08-29 00:09, 01:34, 09:50
```

**The 194-byte shape reached the real client and the client parsed it** [L]: those three runs
answered with **13 269 / 5 416 / 1 960** `0x02FF CLIENT_MOB_MOVE` packets, which the client
only sends for a mob it has built and is simulating.

Where the fields actually are now, by aligning a 137-byte and a 194-byte body of the same
template (delta +57):

| field | 137-byte shape | 194-byte shape | value in today's smoke run |
|---|---|---|---|
| `move_action` (`action*2 + facing`) | byte 35 | **byte 92** | **2** — the safe value |
| `appear_type` | byte 41 | **byte 98** | **-2** (`APPEAR_ALREADY_THERE`) |
| hp | offset 50 | **offset 107** | 45, matching the log line |

Byte 35 in the new shape is inside the newly-populated stat block and happens to be `0`, which
is why the check reports the crash condition while the actual action byte is correct. The hp
check is worse: it filters to `len == 137` before reading, so with 194-byte bodies it has
**nothing to check** and fails on the empty list — a check that cannot pass and cannot fail for
the right reason.

### 4.3 What to change (NOT DONE — `tools/channel_smoke.py` was left alone)

Another agent may own this file, so this is recorded rather than wired. Three edits:

1. `plant_character_and_migration`: add `con.execute("UPDATE accounts SET is_gm = 1")` after
   the `account_id` lookup. Better still, drive **both** cases — a second pass with `is_gm = 0`
   asserting that `!map 40` comes back as one `0x0231` and moves nobody, which is now a real
   behaviour with no coverage at all.
2. Replace the three hard-coded mob offsets with offsets derived from
   `net::mob::MOB_ENTER_FIELD_LEN` and the stat block, or parse the body rather than indexing
   it. A test pinned to a byte index is exactly the "constant that came from reading a header"
   this repo has been bitten by.
3. `four are NpcEnterField` should assert the **templates and the map**, not the count. It
   passed today while sending map 1's NPCs twice.

---

## 5. What the archive contradicts

### 5.1 The redaction commit defeated this project's own de-duplication rule [L]

CLAUDE.md says:

> `research/fixtures/skill-window-close-faults-world.log` and
> `previous-runs/world-20260820-181822.log` are **the same run**, and hash differently - they
> differ at character 74 of line 1 and by 41 KB of tail, **because a fixture is copied while the
> run is still being written**. **Eleven such pairs.**

The pair is real and the same run. **The stated cause is not what happened.** Measured today:

```text
previous-runs/world-20260820-181822.log            8 958 921 bytes   40 894 lines   0 CR bytes
research/fixtures/skill-window-close-faults-...    8 999 815 bytes   40 894 lines  40 894 CR bytes
                                                   ---------
                                                      40 894  = exactly one CR per line
```

Line 1 is 74 bytes in one and 75 in the other; the extra byte **is** the carriage return. The
"41 KB of tail" is 40 894 line-ending bytes. After `CRLF→LF` and the two MAC substitutions the
two files are **byte-identical**. No truncation, no mid-write copy.

The real cause is commit `4f6b448` (2026-08-28 13:30), *"Redact the capturing machine from
every capture"*, which rewrote every committed capture in text mode. Sweeping all 219 fixture
`.log` files against `previous-runs/`:

```text
exact  (byte-identical)                        20
redact (identical after CRLF->LF + MAC subst)  45     <- these are the "pairs that hash differently"
prefix (a genuine mid-write truncated copy)     0     <- none exist today
none   (original has rolled out of the buffer) 154
```

The `prefix` check is in the tool and found nothing, so the mid-write phenomenon may have
happened once and is not what these pairs are. Two things follow:

* **The event-dedup numbers CLAUDE.md quotes are fine** — I re-measured 151 648 mob-moves from
  268 732 raw lines and 276 user-hits from 461, the same shape as its 131 003 / 243 418 and
  257 / 435 with three runs' growth. The *count* claim stands. The *causal* claim does not.
* **`tools/extract_attack_bodies.py --dedupe` is now structurally unable to do its job.** It
  de-duplicates on **file content hash**, and the redaction guarantees a different hash for 45
  of the 65 pairs. Today it reports **787 attack bodies**; event-level de-duplication reports
  **527**:

  ```text
  849  raw lines, no dedup
  787  file-hash dedup   <- what the tool prints today, ~49% over
  527  event dedup on (timestamp, opcode, body)
  527  independent cross-check: distinct bodies alone
  ```

  The per-skill census, event-deduplicated: **454** plain melee (skill 0), **35** skill
  1001001, **31** Magic Claw (2001003), **6** skill 1001002, **1** Three Snails (1000).
  The Three Snails count matching CLAUDE.md's own corrected `1` exactly is the control that
  says the dedup is doing what it claims.

  CLAUDE.md points at that tool as *the idiom* for this rule. It needs the same
  `CRLF→LF` + un-redact normalisation before hashing, or to stop hashing files and key on
  events. `tools/archive_events.py --unredact` does the latter.

### 5.2 "Exactly one `0x010E` in the whole run" is no longer true

`crates/net/src/useitem.rs` and `session/consume.rs` both say it. It was true of the capture
they were written from. `previous-runs/world-20260821-202046.log` has **two**, 2m37s apart,
because the first was answered. The doc blocks should say *"in the capture that motivated
this"*, and the fix now has a positive observation behind it (§1).

### 5.3 T5 will produce a false negative if it is run on the second account [L] — **read this before the next launch**

The database today:

```text
char  name      account          is_gm   map
206   Tester    1  maplecw          1     1          Maple Island
212   Idiot     1  maplecw          1     10000000   Lith Harbor
213   Cobalt    1  maplecw          1     10000000   Lith Harbor
214   Tester2   2  tester           0     40         Maple Island
```

**`Tester2` is the only character on the second account, and that account is not a GM.**
T1 and T2 need two clients, so the second player is `Tester2`. Every `!` command typed on that
client is now **said out loud as an ordinary chat balloon** and does nothing.

T5 opens with `!item 2030000 3`. On the second client that produces a chat balloon reading
`!item 2030000 3` and **no scroll**. The plan's own reading of "nothing happens at all" is
*"the client never sent `0x010E` for a `0203` item, and the whole path is dead code"* — which
would be flatly wrong, because the scroll would never have been in the bag.

Two ways out, either is enough:

* run T5 on `Tester`/`Idiot`/`Cobalt` (account `maplecw`, `is_gm = 1`) and say so in the step, or
* grant the second account GM before the run.

And the *symptom is diagnosable*, which is worth telling the owner: **if a `!` command comes back as
a chat balloon showing the command text, that is the GM gate, not a broken command.**

### 5.4 T5's El Nath expectation is conditional on which continent the character is standing on

T5 says the El Nath scroll *"must REFUSE"*. That is right for all four characters as stored
today — every one is on Maple Island or Victoria Island. But `2030009` goes to `20001000`,
which is **Ossyria**, and `previous-runs/world-20260828-142900.log` left a character on **map
20000000, Orbis** — also Ossyria. From there the scroll is *same-continent* and the guard
should **allow** it. One sentence in the step ("do this from Lith Harbor or Maple Island")
removes the ambiguity; without it, a correct allow reads as a broken guard.

### 5.5 The "675 014 archived events" figure does not reproduce

`tools/test-server.ps1` T4 says *"advance_job_for has NEVER been observed to run - zero
occurrences in 675 014 archived events"*.

**The claim is true** [L]: `"You are now a"` appears in zero archived logs, and the positive
control for that search — `"is not a command"`, a server-sent chat string of the same shape —
is found in 6 files. `advance_job_for` also only landed in commit `a2ffa46`, so no archived run
could contain it.

**The number is not reproducible.** Today the archive holds 339 619 distinct events, 593 585
raw packet lines, and 1 129 647 total log lines. 675 014 is none of these, and it cannot be a
smaller *raw* count from 2026-08-28 because the archive has only grown. Whatever counted it
counted something else. Not load-bearing — the claim rests on the string search, not the
denominator — but a number quoted in the test plan should be a number someone can get back.

---

## What did not close, and what it would cost

| question | state | cheapest next step |
|---|---|---|
| does the client send `0x010E` for a `0203` item? | **open** — opcode live, item class never tested | `!item 2030000 1` on the GM account, double-click, grep `world.log` for `0x010E`. One action inside a run already happening. |
| will the client run twice on one machine? | **open** — never attempted | launch `client-patched\MapleStory.exe` twice, no server, watch 10 s. Costs no elevated launch. |
| do session tokens reach the wire on any opcode other than `0x0073`? | **open** — the 2026-08-18 capture truncates bodies at 16 bytes | re-run the token launch against today's login server, which logs full bodies. Only worth a launch if the token route is being reconsidered; the static Xref negative is the stronger half. |
