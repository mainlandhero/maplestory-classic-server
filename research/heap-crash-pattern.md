# What the heap deaths have in common — the whole archive, one row per death

Nobody had ever asked the archive *"what do these crashes have in common?"*. This is that
question, answered from `previous-runs/`, `research/fixtures/` and the repo root, with no
client run, no Ghidra and no dump opened.

Labels: **[L]** measured · **[D]** derived from something measured · **[I]** inferred.

---

## The short version

* **There are 14 heap deaths in the archive, not 10.** `STATUS.md:298` says *"Across every
  archived run: 10 deaths, 8 at one site, 2 at the other."* Event-deduplicated the count is
  **14: 10 at `0x14019b58e`, 3 at `0x14019bbf3`, and one whose hook log predates the fault
  handler entirely** and is known only because the exit watcher recorded the exit code. `[L]`
* **It is the session, not the thing — and the archive can prove it with exposure.** Across
  42 field-reaching sessions and 3.26 client-hours, the heap hazard is **0.00 per client-hour
  in the first 100 s** and **≈9.3 per client-hour after 200 s**. 3 831 client-seconds were
  spent in the 0–100 s band and **not one heap death happened there**; at the steady-state rate
  that band should have produced ~10. `P ≈ 5×10⁻⁵`. **The earliest heap death in the archive is
  at 192 s of client life.** `[L]`
* **Field entry is where the damage is *noticed*, not where it is *made*.** 4 of the 14 died
  **inside** the `0x01A0 SetField` handler (no dispatch-return line — the hook writes those on
  return). But **90 SetField handlers completed inside the first 100 s of a session and none of
  them killed the client**, against 4 fatal out of 77 late ones (Fisher one-tailed `p = 0.043`).
  A field entry is only dangerous in a client that is already ~3 minutes old. `[L]`
* **Map `10000002` is exonerated as far as the archive can exonerate anything.** It has been
  entered **three** times, and only the oldest client died there. The 2026-08-21 entry — same
  map, same `"in01"` portal, same portal id 2 — happened at **129 s** of client life, completed,
  and the client played on for **8.5 more minutes and two more maps** before dying elsewhere.
  A third entry landed on disk **while this sweep was running** (2026-08-29 23:15, character
  select straight onto `10000002` at **8 s** of client life): **no heap fault**. §6. `[L]`
* **The C++ throw counter does not discriminate and should not be used.** It reaches **19 in a
  59-second session that never crashed**, and one heap death fired at **3**. It is dominated by
  a startup burst: tonight's 30 were 28 in the first 63 s, then one at 358 s and one 5 ms before
  the fault. `[L]`
* **Nothing accumulates that a process-level instrument can see.** Working set and handle count
  are **flat** across every heap death (`~370–430 MB`, `~1440–1450 handles`) after the initial
  asset load, and identical to the non-crashing controls. `[L]` A corrupted free-list header is
  a few bytes; this rules out a leak, not a corruption.
* **A new handover for the dump work:** the five dumps' damaged-slot counts fit
  **one slot per ~300 seconds of IN-FIELD time** within ±30 %, where "per ~350 s of *process*
  time" is off by 50 % on the longest run — which is precisely the run
  `research/heap-corruption-2026-08-27.md` used to declare the rate falsified. That run spent
  **381 of its 1 046 seconds sitting at character select.** §7 has the arithmetic and a
  pre-registered prediction for tonight's dump: **2 damaged slots.**

---

## 1. Instruments, and the controls that could have failed

Everything ran with the repo as the working directory, piped in as `python - < script` so
`sys.path[0]` is empty — the scratchpad shadows `tools/` otherwise. Scripts are in
`C:\MapleCW\scratchpad\pattern-agent\`.

| instrument | what it does | control | result |
|---|---|---|---|
| `faults.py` | walks every `.log`/`.txt` under the repo, keys faults on `(timestamp, code, address, stack line)` | must find tonight's known fault and must collapse the `.claude/worktrees/` checkout copies | 28 distinct faults from 1 000 files; every known fixture present `[L]`. A 29th, a `0xC0000005`, arrived at 23:15 while the sweep was running — §6 |
| `survivors.py` | the same sweep for **non**-crashing sessions | the crash table is meaningless without it | 96 hook logs → **71 distinct sessions** `[L]` |
| `hazard.py` | deaths **per exposed client-hour** per age band | a raw death histogram cannot separate "died at 6 min" from "the owner quit at 2 min" | see §4 |
| `earlyband.py` | is the 0–100 s band actually *in the field*? | if the client were still loading there, its zero would be an artifact | median **11.9 s** from hook install to the first SetField; **90 SetFields land inside the first 100 s** `[L]` |
| `exitcensus.py` | `client-exit.log`'s own `EXIT code` line — a **second, independent** instrument | CLAUDE.md's "count the same event in two logs" | **it found a death the hook log had not** — see §2 |
| `mapexposure.py` | field entries per map, event-deduplicated, parsed by SetField *kind* | asserts the sweep finds ≥100 SetFields before reporting any per-map number | **233** distinct SetField events `[L]` |
| `tools/archive_events.py --unredact` | opcode census | its own docstring's blind spots | 251 `0x00DC`, 252 `0x02DE`, 256 `0x01A0` `[L]` |

**Deduplication is on EVENTS, never files.** `research/fixtures/` holds copies, commit
`4f6b448` rewrote the MAC in the committed ones so hashes differ, and `.claude/worktrees/`
holds a third copy of the whole fixture tree. Every count here keys on
`(timestamp, line content)`.

**A control that killed a red herring before it got into this file.** `0x02DE` (client→server,
1 byte `01`) is the *last packet* in both the amherst and the potion-shop captures, which reads
like a signature. It is not: there are **252** distinct `0x02DE` against **251** distinct
`0x00DC`. It is sent once per field entry, always. `[L]`

---

## 2. The archive has 14 heap deaths, and one of them is invisible to the hook log

`research/fixtures/amherst-1013-heap-corruption-exit.log`:

```text
22:31:08.189 EXIT code 0xC0000374 (-1073740940) after 193.5s of life, 28.34s of CPU
```

Its hook log, `research/fixtures/amherst-1013-heap-corruption-hook.log`, contains the string
`CLIENT FAULT` **zero times** — the vectored handler did not exist yet when that run was made
(it is 2026-08-19; the run is `previous-runs/world-20260819-223107.log`, maps
`40, 80001000, 90000000, 90040200, 90040203, 90050000, 1010, 1013`). `[L]`

Two consequences, and the second is the one that matters:

* The fault-line census is **complete from 2026-08-20 onward** and can silently miss anything
  before it. Only three archived runs predate that, so the undercount is bounded at three. `[D]`
* **The two instruments agree 6 of 7 times and disagree once.** The exit watcher saw seven
  `0xC0000374` exits; six have a matching `CLIENT FAULT` line and one does not. Neither log is
  sufficient alone. `[L]`

And a fixture-name lesson this file owes to CLAUDE.md: `amherst-1013-heap-corruption-hook.log`
is *named* for a heap corruption and does not *contain* the word. Two passes over filenames
would have counted it; a grep of contents finds it only in the exit log beside it.

---

## 3. The table

One row per distinct heap death, oldest client first. Blanks are things the archive cannot say.

| # | fault (local) | run | age | in-field age | addr | map at death | Δ since SetField | what the player was doing | throws | dispatches |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 2026-08-19 22:31:08 | `amherst-1013-*` | **192 s** | 186 s | *(no stack — exit code only)* | 1013 | **0.71 s** | portal `"in02"` → 1013; **died inside the handler** | 13 | 1 117 |
| 2 | 2026-08-20 22:46:38 | `world-20260820-224638` | 210 s | 202 s | `0x14019bbf3` | 40 | 119 s | mob moves + idle NpcChat | 4 | 5 783 |
| 3 | 2026-08-21 02:11:58 | `world-20260821-021158` | 239 s | 215 s | `0x14019b58e` | 40 | 82 s | **equipped a hat** (`0x0107` → `0x0070`), fault 10 ms later | 8 | 2 339 |
| 4 | 2026-08-29 09:50:43 | `world-20260829-095043` | 268 s | 253 s | `0x14019b58e` | 10000000 | 202 s | **idle** — NpcChat chatter only | 15 | 2 546 |
| 5 | 2026-08-28 00:01:34 | `world-20260828-000135` | 269 s | 262 s | `0x14019bbf3` | 10000000 | 13 s | idle after a revive **out of** 10000021. **`-HeapFix` was ON** — the only such run | 21 | 2 461 |
| 6 | 2026-08-22 08:26:45 | `world-20260822-082646` | 304 s | 293 s | `0x14019b58e` | 10 | **0.33 s** | **character select → `GoodTest`**; died inside `SetField`, before `0x00DC` | 15 | 4 636 |
| 7 | 2026-08-28 13:47:01 | `world-20260828-134701` | 337 s | 324 s | `0x14019bbf3` | 1014 | 250 s | mob moves | 3 | 6 063 |
| 8 | 2026-08-21 11:47:10 | `world-20260821-114710` | 378 s | 371 s | `0x14019b58e` | 30 | 44 s | walking, mob moves (Jr Sentinel turn-in run) | 10 | 321 |
| 9 | 2026-08-20 22:24:40 | `world-20260820-222440` | 387 s | 379 s | `0x14019b58e` | 40 | 379 s | mob moves, no map change all session | 4 | 10 633 |
| 10 | 2026-08-27 21:09:54 | `world-20260827-210956` | 402 s | 365 s | `0x14019b58e` | 10001050 | **0.45 s** | GM `!map 10001050`; died inside `SetField`, 8 ms after `0x00DC` | 17 | 4 657 |
| 11 | **2026-08-29 22:48:14** | *tonight* | 537 s | 526 s | `0x14019b58e` | 10000002 | **0.42 s** | portal `"in01"` → 10000002; died inside `SetField`, 3 ms after `0x00DC` | 30 | 4 226 |
| 12 | 2026-08-21 20:20:46 | `world-20260821-202046` | 593 s | 579 s | `0x14019b58e` | 40 | 102 s | mob moves | 23 | 11 061 |
| 13 | 2026-08-21 23:01:02 | `world-20260821-230104` | 640 s | 634 s | `0x14019b58e` | 10000010 | 123 s | mob moves | 15 | 4 009 |
| 14 | 2026-08-27 22:47:23 | `world-20260827-224723` | **1 045 s** | 663 s | `0x14019b58e` | 10005000 | **661 s** | **essentially nothing** — see below | 9 | 392 |

Blanks worth naming: **row 1 has no fault address at all** (its hook log has no handler), and
**no row has a launch command line** — no archived log records one, so `-HeapFix` is only
visible because the hook prints a patch line when it is on. `[L]`

### Row 14 is the single most useful row in the table

`previous-runs/world-20260827-224723.log`, whole-session opcode census `[L]`:

```text
  318  0x0453  NpcChat idle chatter (server -> client, ~1 per 3.3 s)
   46  0x013D  client telemetry
   42  0x007C  StatChanged - idle regen
   38  0x02F4  client telemetry
   19  0x02D1  UserEffectLocal
   18  0x0070  inventory ops
    7  0x044F  NpcEnterField
    4  0x00D9  CLIENT_USER_MOVE      <- the player moved four times in 17 minutes
    3  0x00DC  CLIENT_FIELD_ENTERED
        0x02FF  --- ABSENT ---       <- no mob under client control, all session
        0x03E4  --- ABSENT ---       <- therefore no MobCtrlAck either
```

No attack packet. No mob. Two field entries, both inside the first two seconds *of in-field
time*. Four movement packets. **The client stood still on map 10005000 for eleven minutes and
died of heap corruption**, and its dump (`158352`) carries two damaged slots.

This is also the run that spent its first **381 seconds at character select** —
`previous-runs/login-20260827-223619.log` stamps `0x0010` login result at 02:30:07 and
`0x0078` character select at 02:36:19 — which is what makes it the natural experiment in §7.

That row eliminates, in one observation, *mobs*, *combat*, *inventory*, *shops*, *NPC clicks*,
*movement* and *map changes* as **necessary** conditions. `[L]`

---

## 4. Verdict on "thing versus session" — **[L] it is the session**

The raw ages are 192 … 1 045 s, median 378 s, and on their own they say nothing: a session also
ends because the owner quit. The answer needs **exposure** — how many client-seconds the archive
actually spent in each age band. Sessions that never reached the field (probe runs, netcheck
tests) are excluded by requiring ≥50 dispatched packets.

**42 field-reaching sessions, 11 720 client-seconds = 3.26 client-hours.** `[L]`

| age band | exposure | heap deaths | heap per client-hour | any fault per client-hour |
|---|---|---|---|---|
| 0–100 s | 3 831 s | **0** | **0.00** | 1.88 |
| 100–200 s | 2 861 s | 1 | 1.26 | 2.52 |
| 200–300 s | 1 943 s | 4 | 7.41 | 14.82 |
| 300–400 s | 1 351 s | 4 | 10.66 | 13.32 |
| 400–600 s | 1 110 s | 3 | 9.73 | 12.98 |
| 600–1 200 s | 623 s | 2 | 11.56 | 17.34 |

Pooling 200 s and above gives **9.31 heap deaths per client-hour**, and it is flat across all
four bands above 200 s. At that rate the 0–100 s band should have produced **9.9** deaths and
produced **zero** — `P ≈ 5×10⁻⁵`. The first 200 s should have produced **17.3** and produced
**one** — `P ≈ 6×10⁻⁷`. `[D]`

**The control that had to be run before believing that**, because a young client might simply
not be in the field yet: median **11.9 s** from hook install to the first dispatched `SetField`,
max 382 s (row 14's character-select park), min 6.6 s. The 0–100 s band is ~88 % in-field. `[L]`

**And the shape is specific to the heap family.** Access violations are *not* absent early. In
the same 42-session field-reaching corpus the earliest `0xC0000005` is at **24 s** and two fall
below 100 s, giving a hazard of **1.88/h in the band where the `0xC0000374` hazard is 0.00**.
Across *all* faults, including the short probe runs the corpus filter drops, **six**
`0xC0000005` have fired under 100 s of client life (10.8, 11.7, 12.9, 14.3, 23.3, 68.2 s) and
**zero** `0xC0000374`. Two fault families, same corpus, opposite shapes. `[L]`

So the honest sentence: **something accumulates for roughly two to three minutes; after that
the client is in a state where the next allocator event that touches the damaged slot kills it,
and that becomes a flat-hazard Poisson process at about one death per six client-minutes.**

CLAUDE.md warns that neither reading is the safe default, and it is right here too — the
*previous* two times this question was asked, the answer was "the session" once and "the thing"
once. This time the exposure table settles it, and it settles it the same way the dump work
did from the other side: `STATUS.md:1902` already records *"damage tracks session age
(rank corr 0.80) rather than map loads (0.05)"*. Two instruments, no shared blind spot.

---

## 5. Verdict on the map and the action — **[D] field entry is enriched, and it is a detector**

4 of the 14 died **inside** the `0x01A0 SetField` handler. All four have no dispatch-return line
for that packet, which is how the hook says "entered and never came back":

| run | SetField sent | `0x00DC` back | death | where in the entry |
|---|---|---|---|---|
| 2026-08-19 amherst | 22:31:07.482 | 07.900 | ~07.90–07.97 | just after `0x00DC`, during `0x044F` |
| 2026-08-22 GoodTest | 08:26:45.119 | *never arrived* | 45.450 | **before** `0x00DC` — died decoding the record |
| 2026-08-27 `!map 10001050` | 21:09:54.508 | 54.946 | 54.954 | 8 ms after `0x00DC`, during `0x044F` |
| 2026-08-29 potion shop | 22:48:14.477 | 14.895 | 14.898 | 3 ms after `0x00DC`, during the `0x0070` restore burst |

**The enrichment is real.** 233 distinct field entries against 11 720 client-seconds; even
allowing a generous 0.75 s window per entry that is **1.5 % of client-alive time carrying 29 %
of the deaths** — about 19× — with `P ≈ 1×10⁻⁴` on a Poisson test. `[D]`

**And it is almost certainly a detection effect, not a cause.** Two things say so:

1. **90 SetField handlers completed inside the first 100 s of a session and not one killed the
   client**, against 4 fatal among 77 late entries. Fisher one-tailed `p = 0.043`. `[L]`
   A field entry is not dangerous; a field entry *in an old client* is.
2. Field entry is the client's largest allocate/free burst — it tears the whole field down and
   rebuilds it. That is exactly where a **pre-existing** bad free-list entry is most likely to
   be reached first. The fault addresses are the client's `free()` and its heap walk; they are
   detectors by construction.

**A caution that must be stated, because it weakens the lead.** The access violations show the
same clustering: three of the fifteen `0xC0000005` faults also have a `0x01A0` in flight within
0.5 s. Field entry is where this client dies *in general*, not specifically where it dies of
heap corruption. `[L]`

Per-map, exposure-normalised, the deaths simply track exposure. Map 40 has 39 entries and 4
deaths; map 1 has 30 and **0**; map 10 has 17 and 1; map 20 has 17 and 0; map 10001050 has 12
and 1; map 1013 has 11 and 1. The overall rate is 14/233 ≈ 0.06 and nothing stands out above
the noise at these counts. `[L]`

> **A parsing trap worth recording, because the first version of this table had it.** A
> SetField line's *last* map id is not its destination. `REVIVE: map 10005075 -> 10005000` and
> `taxi 400 … sending them to map 10000000 (Lith Harbor)` both put the source or a parenthetical
> where a naive `map (\d+)` grabs it. 6 revives, 2 taxis and 5 cash-shop exits out of 233 are
> mis-attributed by that regex, and it moved two rows of the death table onto the wrong map.
> Parse the destination per SetField *kind*: `charselect` 72, `portal` 86, `gm` 61, `revive` 6,
> `taxi` 2, `cash-shop exit` 5, unparsed 1. `[L]`

---

## 6. Map `10000002` — **[L] exonerated, on three entries that vary only by age**

`10000002` (Lith Harbor potion shop interior) appears **three** times, and they line up on the
one axis §4 says matters:

```text
2026-08-21 22:52:31   client age  129 s   portal "in01" -> 10000002 portal 2, character 212
                      SURVIVED. Left through "out01" at 22:58:51 -> 10000000, then "east00"
                      -> 10000010, and died there at 23:01:02 - 8 min 31 s later, elsewhere.
2026-08-29 22:48:14   client age  537 s   portal "in01" -> 10000002 portal 2, character 213
                      DIED, 0xC0000374, inside the SetField handler, 421 ms later.
2026-08-29 23:15:01   client age    8 s   character select carrying map 10000002, character 213
                      NO HEAP FAULT.
```

**Same map, same portal, same portal id, and the only thing that varies is the client's age.**
`[L]` The 2026-08-21 entry at 129 s completed and the client played on for eight and a half
minutes. That is exactly the threshold §4 measured, and it is the whole explanation the archive
is able to offer.

**The third entry arrived on disk during this sweep and is worth stating carefully.**
`client-patched\maplecw-hook.log` (23:14:53 install) shows a fresh client entering `10000002`
at character select, 8 seconds old, and dying **12.9 s in with `0xC0000005` at `0x140ce89d6`** —
`dumps\maplecw-crash-647420-c0000005-1.dmp`. That is **not** this family: `0x140ce89d6` has
**five** prior appearances in the archive, on five different maps and five different actions
(`setfield-accepted-client-entered-world`, `shop-one-row-still-faults`,
`npc-action-byte-swap-kills-client-on-sera-chatter`, `classic-shop-opens-sell-crashes`,
`channel-migrate-is-0x001a`). Four of those carry a stack, and all four share the same core —
`0x14308e3ed 0x143ae2d20 0x142f04320 0x14308e3db 0x14374e6fc 0x142efba4e …` — differing only in
the leading frame and the last one or two. **Near-identical, not byte-identical**, and worth
saying that way. So it tells us nothing about the map — but it does say, positively, that
**a young client can enter `10000002` and the heap check does not fire.** `[L]`

It also corroborates §4 from the other side: it is the **sixth** `0xC0000005` under 100 s of
client life, in a band where `0xC0000374` has still never once fired.

The one thing `10000002` genuinely is: **an interior with an inventory-restore burst**, and
tonight's fault landed in the middle of it. But that burst fires on every field entry — 7 746
distinct `0x0070` events across the archive — so it is not special either. `[L]`

**Do not go looking for what is different about map 10000002.** This is the same shape as the
mobs-versus-NPCs comparison CLAUDE.md records: three days spent on *"find the difference
between A and B"* when nobody had established that A and B differ.

---

## 7. The two fault addresses — **[L] different sites, [D] same event**

| address | n | what it is | who says |
|---|---|---|---|
| `0x14019b58e` | 10 | `FUN_14019b4e0 + 0xae` — the return address of the `call rbx` that **is** `HeapFree`, in the client's pooled `free()` | `research/equip-crash.md:45`, `research/heap-corruption-2026-08-27.md:30` |
| `0x14019bbf3` | 3 | `FUN_14019bb6a + 0x89` — a **second** pooled free, reached through `0x140205833` / `0x1401be180`, compile-time-fixed route | `research/heap-corruption.md:43`, `research/heapfix-did-not-hold.md:22` |
| *(none)* | 1 | amherst — no handler in that build | §2 |

They are **two call sites into the same allocator**, 1 637 bytes apart, and both are the point
where the *already damaged* header is read. All three `bbf3` stacks carry `0x140205833` and
`0x1401be18x` and none of the `b58e` stacks reach it that way, so they are genuinely different
routes into the same failure. `[L]`

They do **not** correspond to different situations. `bbf3` fired on map 40 mob-moves (2026-08-20,
`-HeapFix` did not exist), on map 10000021 idle with `-HeapFix` **on** (2026-08-28), and on map
1014 mob-moves with it **off** (2026-08-28). Age, activity and map spread the same way as the
`b58e` set. `[D]`

### The handover to the dump work: the clock may be **in-field** seconds, not process seconds

`research/heap-third-dump.md:109` and `research/heap-corruption-2026-08-27.md:4` give the five
damaged-slot counts. Adding the in-field age (process age minus the time to the first
`SetField`, which the world log dates exactly):

The in-field age here is measured directly as *(fault timestamp − the run's first
`0x01A0 SET_FIELD`)*, both off the same clock, rather than by subtracting two ages that come
from two different watchers. The middle column is the remainder — login, character select, and
the ~2 s between the exit watcher attaching and the hook installing.

| dump | process age (as published) | not in a field | **in-field age** | damaged slots | s/slot, process | s/slot, **in-field** |
|---|---|---|---|---|---|---|
| `1224132` (2026-08-22 08:26) | 306 s | 13 s | 293 s | 1 | 306 | **293** |
| A `155108` (2026-08-27 21:09) | 403 s | 38 s | 365 s | 1 | 403 | **365** |
| (2026-08-21 20:20) | 596 s | 17 s | 579 s | 2 | 298 | **290** |
| (2026-08-21 23:01) | 644 s | 10 s | 634 s | 3 | 215 | **211** |
| B `158352` (2026-08-27 22:47) | 1 046 s | **383 s** | 663 s | 2 | **523** | **332** |

Process-time rate: mean 349 s/slot, range 215–523, coefficient of variation **0.34**.
In-field-time rate: mean 298 s/slot, range 211–365, coefficient of variation **0.19**. `[D]`

The **rank** correlation is unchanged — both clocks order the five dumps identically, so
Spearman is 0.80 either way, which is exactly the number `STATUS.md:1902` already reports. What
changes is the *rate*, and the rate is the thing that was declared falsified.

The whole difference is dump B, and dump B is the run that **sat at character select for 381 of
its 1 046 seconds** — `previous-runs/login-20260827-223619.log` timestamps it: `0x0010` login
result at 02:30:07, `0x0078` character select at 02:36:19, six minutes later. That is the exact
run `research/heap-corruption-2026-08-27.md:25` used to write *"the one-per-250-s rate does not
survive"*. **On the in-field clock it survives fine.** `[D]`, `n = 5`, so this is a candidate,
not a finding — but it is a *free* candidate and it makes a sharp prediction:

> **Tonight's dump (`maplecw-crash-627340-c0000374-1.dmp`, in-field age 526.0 s, process age
> 537 s) should carry exactly 2 damaged slots.** The observed in-field envelope, 211–365 s/slot,
> admits only 2 at 526 s (1.44 … 2.49). **A count of 1 or 3 puts the rate outside every previous
> point; 0 or ≥4 refutes the in-field model outright.** Tonight is a weak discriminator between
> the two clocks — the run barely parked, so process and in-field agree — which is exactly why
> it is a fair test of the *rate* rather than of the clock.

Written down before the dump is opened, deliberately. `[I]`

---

## 8. What the throw counter is worth — **[L] nothing, as a predictor**

The brief guessed this might be the most useful number in the log. It is not:

```text
heap deaths, throws at fault:   3  4  4  8  9 10 13 15 15 15 17 21 23 30
sessions that never crashed:    up to 19 throws (previous-runs\maplecw-hook-20260822-205640.log,
                                59 seconds of life)
an access violation:            32 throws (previous-runs\maplecw-hook-20260829-000859.log)
```

The distributions overlap completely, and the counter is dominated by a startup burst — of
tonight's 30, **28 arrived in the first 63 seconds**, then one at 358 s and one 5 ms before the
fault. It measures how much exception-based control flow the client did while loading, not how
damaged the heap is. `[L]`

Likewise the **last returned opcode** before the fault (`0x03E4` seven times, `0x0453` four
times) is pure base rate: those two are the most common packets in every log. `[L]`

And a genuine negative that is worth having: **working set and handle count are flat.**

```text
setfield-died-inside-0x01a0-goodtest   7s:257MB/h1418  63s:408/h1449  187s:428/h1444  305s:473MB/h1449
heap-corruption-2-map40-30mobs         7s:257MB/h1436  78s:380/h1451  234s:376/h1444  384s:374MB/h1445
jr-sentinel-turnin-heap-corruption     7s:418MB/h1437  79s:360/h1453  231s:363/h1442  380s:369MB/h1447
--- control, no heap fault ---
classic-shop-opens-sell-crashes        6s:416MB/h1435  53s:410/h1453  146s:400/h1446  239s:417MB/h1442
```

Whatever accumulates is **not** an allocation and **not** a handle. `[L]` That is consistent
with a stray one-byte-ish write into a header and inconsistent with a leak.

---

## 9. The cheapest experiment, and what each outcome means

Everything above closes without a launch. What remains open is **what the clock is** — wall
time, in-field time, or something the client does at a constant rate — and §7 says the archive
already leans one way by accident. One run makes that deliberate, changes exactly one thing
against tonight, and cannot come back ambiguous.

> **Launch, get to the character-select screen, and then leave it alone for twelve minutes
> before selecting the character. Then play normally — anything at all — until the client dies.**
> Everything else identical to tonight's run: same character, same flags, `-HeapFix` off.

Twelve minutes is chosen so that the process clock is **already past every death age in the
archive** at the moment the character enters the field. **That makes the readout minutes, not a
one-slot difference in a dump** — which matters, because at these counts a slot is a coin-flip.

**The primary reading is the wall clock, and the owner can take it off their own screen:**

| what happens | what it means |
|---|---|
| **dies within ~60 s of entering the field**, i.e. on or right after the first `SetField` | the accumulator ran during the twelve minutes at character select, where there is no field, no mob, no NPC and almost no server traffic. The clock is **process time**, and that eliminates nearly the whole game loop in one run. |
| **survives entry, plays for 3+ minutes, then dies at a normal in-field age** (the archive's in-field ages at death are 186…663 s, median 371) | the twelve-minute park bought nothing and cost nothing. The clock is **in-field time**, and the next hunt is per-frame or per-field-tick work rather than a process timer. |
| **survives more than twelve minutes in the field** | the first archived session to pass 600 s of in-field time without a fault — 3 of 3 previously died. Report the number; both models are then wrong and the park becomes a candidate *protection*. |
| dies of `0xC0000005` instead | not this family — six of those have fired under 100 s of client life and one fired tonight at 12.9 s. Re-run; it costs one launch and says nothing either way. |

**The secondary reading is free and sharpens whichever branch happens.** Run
`python tools/poolchain.py` (from `C:\MapleCW`, never from a scratchpad) on
the dump the hook writes, and take the in-field age off `world.log` as *first
`0x01A0 SET_FIELD` → the fault* — the world log is UTC and the hook log local, so compare
`MM:SS.mmm`. Under the in-field model the count is `round(in-field seconds / 298)`; under the
process model it is `round(process seconds / 349)`, and after a twelve-minute park those two
predictions differ by 2 slots or more instead of by one.

**The procedure has a positive control and it costs nothing to state:** the 2026-08-27 22:47
run did exactly this by accident — parked **383 s**, entered the world, and then survived
**663 s in the field** before dying. So the park itself is known not to break the client, and
at `n = 1` the archive already predicts branch two. `[L]`

Two things **not** to do first, both for the reason CLAUDE.md gives about starting the static
work before the experiment:

* Do not go looking for what is special about map `10000002` — §6.
* Do not re-run `-HeapFix` to "see if it helps". It has been on for exactly one archived run,
  that run died anyway at `0x14019bbf3`, and `research/heapfix-did-not-hold.md` explains why it
  structurally cannot cover that site. Every dump taken with it on is also unusable for the
  free-list argument.

---

## 10. What the archive contradicts

1. **`STATUS.md:298` — "Across every archived run: 10 deaths, 8 at one site, 2 at the other."**
   Event-deduplicated across the whole repo the count is **14 deaths: 10 at `0x14019b58e`,
   3 at `0x14019bbf3`, 1 with no address** (2026-08-19, before the fault handler existed).
   `[L]` The sentence's *argument* — that a client died at `bbf3` unpatched, so `-HeapFix`
   cannot be the whole story — is unaffected and still right.

2. **`research/heap-corruption-2026-08-27.md:25` — "The one-per-250-s rate does not survive.
   1 046 s produced 2 damaged slots, not the ~4 the rate predicts."** That run spent **381 s at
   character select**; on in-field time it produced 2 in 663 s, which is *within 15 % of the
   rate the other four dumps average* (332 s/slot against their 290). `[D]` The rate may not be dead — the clock may have been
   wrong. §7.

3. **`STATUS.md:1902` — "`-HeapFix` … has never been switched on in a crashed run."** It was:
   `previous-runs/maplecw-hook-20260828-000135.log` logs `heapfix=on` and the `0x14019b504`
   patch line, and that client died of `0xC0000374` at `0x14019bbf3`. `[L]` This is already
   retracted at `STATUS.md:290`, at the top of the same file — the stale sentence is 1 600
   lines below the retraction and will be read as current by anyone who greps.

4. **A fixture whose name is a claim its own file cannot support.**
   `research/fixtures/amherst-1013-heap-corruption-hook.log` does not contain the words
   `CLIENT FAULT`; the death is only in the `-exit.log` beside it. Anyone counting heap deaths
   by hook log misses it, and anyone counting by filename counts it for the wrong reason.

5. **Nothing here contradicts `research/heap-corruption-2026-08-27.md`'s core findings** — the
   identical `0x0000000100000020`, the `0x20` size class, the free-listed damaged slot. Those
   came from an exact pool enumeration and this sweep has no instrument that could disagree
   with them.

---

## 11. Blind spots of this sweep, stated so nobody mistakes them for absence

* **No archived log records a client command line.** `-HeapFix` is visible only because the
  hook prints a patch line. Any other launch flag that varied between runs is invisible here,
  and would be a silent confound in the exposure table. `[L]`
* **The 42-session exposure spans eleven days and many server builds.** The hazard is treated
  as stationary because heap deaths appear on 08-19, -20, -21, -22, -27, -28 and -29 with no
  gap, but that is an argument, not a measurement. `[I]`
* **`ndisp ≥ 50` is the "reached the field" filter.** It is a proxy. A session that reached the
  field and dispatched 40 packets would be excluded and would add exposure to the young bands —
  which would only strengthen §4's conclusion, so the bias is in the safe direction. `[D]`
* **The "in-field age" column assumes the world log's first `SetField` marks field entry.** It
  does; the client answers with `0x00DC` ~420 ms later, every time. But the world log's clock is
  UTC and the hook's is local, and every cross-file subtraction here was done on `MM:SS.mmm`
  with the hour discarded. Two events an exact hour apart would collide. None do in this corpus,
  and every row was checked against the hook's own session length. `[L]`
* **The one thing the archive genuinely cannot answer** is whether the writer is ours or the
  client's. Row 14 narrows it to "something that runs while the client is in a field with no
  mobs and a motionless player", and the only server traffic there is idle `NpcChat`, idle-regen
  `StatChanged` and `UserEffectLocal`. Separating those from the client's own loop needs a run
  with those sends suppressed, and no launcher switch exists for them today.
