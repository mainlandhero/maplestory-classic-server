# Is the heap corruption something *we* do? What the archive can and cannot answer

The owner, 2026-09-06: *"I just want to make sure that the actual game does not crash like our
client does. There must be something we're doing that's causing this corruption."*

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

This file answers a different question from `research/heap-corruption-2026-09-06.md`, which
asks *what* the damage is. This one asks *whose fault it is*, and its most important sentence
is a negative: **there is no unhooked control run anywhere in this project's history**, so the
comparison the owner is asking for has never been made and cannot be made from the archive.

---

## 0. The short version

| claim | tag |
|---|---|
| Retail cannot be measured from here - no retail server, and **75 of 75 archived client runs carried our hook** | `[L]` |
| `identity.rs` and `instance.rs` are **exonerated by date** - the family is 11 and 13 days older | `[D]` |
| `hook.rs`, `probe.rs` and `session.rs` predate the first heap death and are **not** exonerated | `[L]` |
| The client already has **one** memory-corruption bug that fires only because of our network environment - the `~36.9 s` reachability `__fastfail` we stub | `[L]` |
| Both sentry catches landed **immediately before the client's 30 s census send**, and were exactly **6 census intervals apart** | `[L]`, and see §4 - two samples are not a period |
| In our sessions the client **clears and rebuilds a global map every frame** because one timestamp field is zero; in the earlier idle session that field was non-zero | `[L]` |
| The damage's *regularity* argues **against** Themida anti-tamper vandalism and **for** a genuine type confusion | `[D]` |

---

## 1. The control that does not exist

```text
previous-runs/  *hook*.log        75
previous-runs/  *client-exit*.log  0
```

`[L]`. Every archived run of this client went through `grap-stub`. There is no session in
which the client ran unpatched, so **no measurement in this project distinguishes "the client
corrupts its heap" from "the client corrupts its heap when we are inside it".** Every rate,
every hazard curve and every dump in the heap family was taken with the hook attached.

That is not an argument that the hook is guilty. It is a statement about what the evidence can
carry, and it is the reason §5's experiment is worth a launch.

## 2. What is exonerated, and it is exonerated by dates rather than by argument

The earliest heap death in the archive is **2026-08-19 22:31, at 192 s of client life**
(`research/heap-crash-pattern.md` row 1). `[L]` Against the day each patch entered the tree:

| module | added | verdict |
|---|---|---|
| `hook.rs` | 2026-08-15 | predates the family - **not** exonerated |
| `probe.rs` | 2026-08-15 | predates the family - **not** exonerated |
| `session.rs` | 2026-08-16 | predates the family - **not** exonerated |
| `netwatch.rs` | 2026-08-17 | predates the family - **not** exonerated |
| `identity.rs` | 2026-08-30 | **11 days younger than the first death - exonerated** |
| `instance.rs` | 2026-09-01 | **13 days younger - exonerated** |

`[L]` for the dates, `[D]` for the verdicts.

The two exonerations matter because both are the kind of thing that *looks* guilty. `identity.rs`
allocates a block from the client's own pool, writes 26 bytes into it and **pins its refcount at
2 so the session destructor cannot free it** - a deliberate leak of a refcounted string, in a
bug whose signature is a refcount-shaped write. It is still innocent of *this*: the family was
eleven days old when that code was written. Same for `instance.rs`, which hooks `FindWindowW/A`
and redirects `CreateMutexW/A`.

**`heapfix` was off** for both sentry catches (`heapfix marker: None` in each finding block).
`[L]`

## 3. What we do that retail does not, ranked

Everything below was active during the sentry session, read out of its hook log. `[L]`

### 3.1 We stub a client function on every launch, and the reason is itself a corruption bug

`watch@1415db360:ret` makes the client return immediately from a network reachability check.
Without it the client `__fastfail`s on a **~36.9 s wall-clock deadline** (`STATUS.md`), and the
mechanism recorded in memory is that **the check overruns its own stack buffer** when the
endpoint is firewalled.

That is the single most important fact for the owner's question, and it cuts the way they are worried
about *and* the other way:

* This client, **in our network environment, already executes one memory-corrupting path that
  retail never reaches.** So "there must be something we're doing" has a proven instance. `[L]`
* But what we *do* about it is suppress the corrupting function. The heap family survives that
  suppression, and it starts at 192 s - long after the 37 s deadline. `[D]`

The honest reading: our environment demonstrably drives this client down error paths it was
not built for. A second such path that corrupts the **heap** instead of the stack is the
hypothesis with the best prior in this file, and nothing here confirms or refutes it.

### 3.2 We never answer three of the client's own reports

Measured over the sentry session, all inbound, all unanswered: `[L]`

| opcode | what it is | count | cadence |
|---|---|---:|---|
| `0x0070` | `CLIENT_ENV_REPORT` | 298 | bursty, roughly 1/s |
| `0x013D` | the 30-second skill-send census | 13 | every ~30.0 s |
| `0x01ED` | `CLIENT_USAGE_REPORT` | 11 | every ~120.0 s |

`research/buffs.md` §2.5 measured 22 unanswered `0x013D` across a ten-minute session with the
client playing to the end, and concluded the server *must not* answer it. That measurement was
about **freezing**, which is what `CLAUDE.md`'s "always answer" rule is about. It says nothing
about heap state, and it should not be quoted as though it did.

### 3.3 The client clears and rebuilds a global map every frame, and a zero timestamp is why

`FUN_142e0f9e0` is the usage-report ticker, called every frame from the game-stage tick
`FUN_142ce0130+0x79a`. Its gate, from the listing: `[L]`

```text
142e0fa40  cmp   dword [r12+0x3b18], 0     ; first call ever? -> just stamp and leave
142e0fa4b  mov   r8d, r13d                 ; tick
142e0fa4e  mov   edx, 0x2bf20              ; 180 000 ms
142e0fa53  mov   ecx, dword [r12+0x39ec]   ; a timestamp
142e0fa5b  call  0x1408fcaa0               ; (tick - [0x39ec]) > 180000 ?
142e0fa62  je    0x142e0fa76               ; NOT elapsed -> build and send the report
142e0fa64  call  0x142d207f0               ; elapsed -> DESTROY the global map
```

`FUN_142d207f0` walks the `std::map<int, std::map<int,int>>` whose head is the global
`0x143ade6b8`, destroys it and resets head and size. `[L]`

Read out of the three dumps with `dumpwalk`: `[L]`

| dump | `[ctx+0x39ec]` | map size |
|---|---:|---:|
| sentry 212108 finding #1 | **0** | 1 |
| crash 212108 | **0** | 1 |
| crash 179092 | 168 186 245 | 1 |

With `[0x39ec] == 0` the subtraction is `tick - 0`, and the client's tick is `GetTickCount`
(173 million ms - about 48 h of machine uptime, not process age). The test is therefore **true
on every frame**, so the client takes the destroy branch every frame and never sends from this
site. The three inserts in the same tick (`keys 0xb/0x5e6`, `0xb/0x603`, `0xb/0x7a0`) refill it
immediately, which is why the map is size 1 in every dump. `[D]`

So in the sentry session the client was **allocating and freeing tree nodes at frame rate**
through the pool, on a branch it should take once every three minutes. Two cautions before this
is treated as the answer:

* The nodes are allocated at `edx=0x38` and `edx=0x28`, which round into the **`0x40` class**,
  and **0 of 693 272 slots outside the `0x20` class have ever been damaged**. So this churn is
  not itself the damaged memory. `[L]`
* `[0x39ec]` is **written to zero by the client's own code** at `142e0fa8b`, so zero is a state
  the client produces, not obviously one we impose. Whether retail leaves it non-zero for long
  stretches is exactly the unmeasurable thing this file is about. `[I]`

The 179092 session had it non-zero and still accumulated eight damaged slots, so this is **not**
a necessary condition for the damage. `[D]`

### 3.4 Themida, and why it is probably not the answer

The client is Themida-packed - `.themida` with `SizeOfRawData = 0`, plus a `.boot` section - and
we run it patched, hooked and with `int3` breakpoints armed, which is precisely what a packer's
anti-tamper is built to notice. A delayed, deliberate memory corruption is a real Themida
behaviour and it would explain the ~192 s onset and the absence of any packet correlation.

It is still the weaker reading, for one reason: **the damage is too regular.** Fourteen
observations, every one of them `0x0000000100000020`, every one in the `0x20` class, every one
at `slot+4`, values `1`, `2` and `-1`. `[L]` Anti-tamper vandalism is indiscriminate; this
writes one field, one width, one place, with values that read as a reference count. A
type-confused refcount write explains that shape and a punishment routine does not. `[D]`

## 4. The one new correlation, and it is a lead, not a finding

The two sentry catches: `[L]`

```text
catch #1  22:34:34.376   the census 0x013D is logged at 22:34:35.101 *
catch #2  22:37:34.414   the census 0x013D is logged at 22:37:34.595
                          * the client was frozen 34.377 - 35.101 writing the dump
```

Both catches land within a few hundred milliseconds **before** a `0x013D` census send, and they
are **180.038 s apart - exactly six 30-second census intervals**. The census builder
`FUN_142d19260` is reached from the tick at `142ce2b45` behind a `0x7530` (30 000 ms) throttle
on `[ctx+0x4088]`, and the same map-destroying `FUN_142d207f0` sits on that path. `[L]`

**Two samples cannot establish a period**, and a 30-second event has a one-in-six chance of
sitting this close to any given moment. `CLAUDE.md`'s whole section on burst-as-object applies:
this is a set to enumerate on the next run, not a conclusion. The prediction it makes is sharp
and cheap - **if the mechanism is census-adjacent, catches will keep landing in the few hundred
ms before a `0x013D`** - and three catches would settle it.

## 5. The experiment that answers the owner's question

One launch, and it is the control that has never been run.

```text
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
  -SetFieldProbe -DirectClient -FallbackAccount maplecw -PoolSentry
  -ClientToken AAAAAAAAAAAAAAAAAAAAAAAAAA
  -Probe "watch@1415db360:ret,141b2a280:rdx=0" -Session "mode=2"
```

### 5.1 Two flags in that line are not client patches, and leaving either out hangs the client

Both were found by handing the owner a command that did not work, twice.

**`-SetFieldProbe`** is a world-server flag. Without it `Session::handle` returns nothing for
every packet and the channel answers nothing at all. The script's `.NOTES` says *"NOT
optional"* and the branch without it prints six red lines naming this exact symptom.

**`-ClientToken`** fills the client's own identity field. This one was not documented anywhere,
and it is the more interesting of the two. Counted over 15 archived login logs: `[L]`

| | carried a token | sent `0x0078` select | got `0x0011` migrate |
|---|---:|---:|---:|
| 9 runs, all through the launcher | 9 | 9 | 9 |
| 3 runs, no token, each sending `0x00C0 CLIENT_AUTH_FAILURE_REPORT` | 0 | **0** | **0** |
| 2 runs that never reached a character list | 0 | 0 | 0 - say nothing either way |

Ten for ten among the runs that reached a character list. The failure is not new: 2026-09-05
21:10 is the same shape as both attempts on 2026-09-06 - character list drawn, Cobalt picked,
`0x00C0` sent instead of `0x0078`, `world.log` never touched. `[L]`

**It is confounded and the flag is the discriminator.** Every token run is a launcher run, so
"no token" and "direct client" cannot be separated by counting. Only the launcher can mint a
token the server accepts, so `-ClientToken` writes one that is **wrong by construction** - and
the login server's own rule is that a wrong token *downgrades* to the fallback account rather
than being refused, so the account served is unchanged. `[L]` If the client then selects a
character, what it checks is its own field. If it still hangs, the token is not the cause and
the direct path differs some other way, which is also a result.

**The identity write is therefore back in the run, and that does not weaken the control.** §2
already cleared it by date: `identity.rs` is eleven days younger than the family's first death.
What this run still tests is `create=on`, the two extra watches and the multiclient hooks.

**`-SetFieldProbe` is not a client patch and is not optional.** It is a world-server flag;
without it `Session::handle` returns nothing for every packet and the client hangs on
"Connecting..." after a character is picked. The first attempt at this run, 2026-09-06 23:27,
did exactly that, because the command written into both plan copies and into this section left
it out. `[L]` - `login.log` shows the character list going out at `03:26:56.690` and then no
`0x0078 CLIENT_SELECT_CHARACTER_REQUEST` at all, against the archived working sequence
`0x0078 -> 0x0011 MIGRATE_COMMAND`, and `world.log` has 35 lines, all of them startup banner.

It does not weaken the control. `-SetFieldProbe` substitutes the default four-watch probe set
**only when `-Probe` was not passed explicitly**, and this command passes it, so the client
still runs with just the two mandatory patches. Everything else the flag touches is the world
server's own arguments and what the plan prints.

The lesson is the file's own: this script's `.NOTES` said *"-SetFieldProbe is NOT optional"* at
line 42 and prints a red six-line warning on the branch without it. A command assembled from
the patch list rather than from the run's requirements dropped it anyway, and cost a launch.

`-DirectClient` is not a detail: **the launcher writes the probe and session markers with its
own defaults**, so the ordinary launcher path cannot run this control at all. That one switch
drops the two extra `int3` watches, `create=on`, the identity write and - after the fix
committed alongside this file - the multiclient hooks. `1415db360:ret` and `141b2a280:rdx=0`
stay; without them the client dies at 37 s and the login dialog blocks. `-PoolSentry` now
writes `dumps=4` rather than the cap of 1 that spent last run's only dump on the first of two
catches.

Then Cobalt, one map with mobs, stand still for fifteen minutes, and **close the client
yourself** - a deliberate end keeps the sample unbiased.

**One stale marker nearly ruined this before it ran.** `maplecw-hook.multiclient` is written by
the launcher and deleted by nothing; the copy in `client-patched\` was twelve days old, so a
`-DirectClient` run would have inherited the `FindWindow` and `CreateMutex` hooks it is supposed
to exclude. `[L]` The script now removes it on that path. A control that quietly carries one of
the things it excludes is not a control, and this one would have looked clean doing it.

| outcome | reading |
|---|---|
| catches at the same rate (~1 per 3 min) | our optional patches are innocent and the bug is the client's own, in our environment. The guard-page build is the only way forward |
| no catches in 15 min | one of the three dropped patches is implicated, and bisecting them is two more launches |
| catches, and each within ~300 ms before a `0x013D` | §4 becomes a finding and the search narrows to the census path |

What it still cannot do is compare against retail. Nothing available here can.

## 6. What could not be established

1. **Whether retail corrupts.** No retail server, no unhooked run, no comparable capture. Not
   inferable from anything on disk.
2. **Whether `hook.rs`, `probe.rs` or `session.rs` contribute.** They predate the family, which
   is not the same as being present at its birth in the causal sense - the family's *first
   observation* is 2026-08-19, and the hook was installed on every run before that too.
3. **Whether `[ctx+0x39ec] == 0` is a state retail reaches.** §3.3.
4. **Whether the census correlation is real.** Two samples. §4.
5. **Whether a second environment-driven corrupting path exists** of the kind §3.1 proves for
   the stack. Not searched; naming it would be a scan of the client's other reachability and
   upload paths.
