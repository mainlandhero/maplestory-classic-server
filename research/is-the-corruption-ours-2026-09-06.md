# Is the heap corruption something *we* do? What the archive can and cannot answer

The owner, 2026-09-06: *"I just want to make sure that the actual game does not crash like our
client does. There must be something we're doing that's causing this corruption."*

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

This file answers a different question from `research/heap-corruption-2026-09-06.md`, which
asks *what* the damage is. This one asks *whose fault it is*, and its most important sentence
is a negative: **there is no unhooked control run anywhere in this project's history**, so the
comparison the owner is asking for has never been made and cannot be made from the archive.

---

> **UPDATE 2026-09-08: §1's "control that does not exist" now exists, and it clears us.** Four
> hook logs came off the LIVE server from other people's machines. One is a crash, and that
> client reported the **identical** damage - `0x0000000100000020` on a `0x20` header - which the
> sentry found and repaired [L]. Different person, different computer, same signature. **The
> writer is not something our environment does to the client.** See
> `research/live-client-crash-2026-09-08.md`.
>
> Everything below stands as the reasoning that was available before that, and §3's list of what
> we do that retail does not is still worth keeping: the reachability `__fastfail` we stub is a
> genuine second corruption path that only our environment reaches, and it is unrelated to this
> family.

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
## 5. The experiment, three failed launches, and a retraction

The control in §1 is still the run worth doing. Getting it launched cost the owner **three client
runs on 2026-09-06** and it has not been launched yet. What those three establish is written
down here because two of them are my errors and one of them is a retraction.

### 5.1 Attempts 1 and 2: a world-server flag that reads like a client patch

The command was assembled from *"which client patches do we drop"* and therefore omitted
**`-SetFieldProbe`**, which is not a client patch at all. It is a world-server flag, and
without it `Session::handle` returns nothing for every packet, so the channel answers nothing
and picking a character hangs on "Connecting to server". `[L]` - `login.log` 03:26:56 and
03:31:29 both show the character list going out and no `0x0078
CLIENT_SELECT_CHARACTER_REQUEST` ever arriving; `world.log` is 35 lines of startup banner.

The script's own `.NOTES` says *"-SetFieldProbe is NOT optional"* at line 42 and the branch
without it prints six red lines naming this exact symptom. Adding it did **not** fix the hang.

### 5.2 Attempt 3, and the retraction: the client token was not it

Between attempts 2 and 3 I counted `0x0078` against the client token over **the 15 most recent
archived login logs** and reported a perfect correlation: 9 of 9 runs carrying a token selected
a character, 3 of 3 without one sent `0x00C0 CLIENT_AUTH_FAILURE_REPORT` instead and never
selected. `-ClientToken` was added to test it.

**Attempt 3 carried the token and hung in exactly the same place.** `[L]` - `login.log`
03:38:49 logs `identity="AAAAAAAAAAAAAAAAAAAAAAAAAA" (26 bytes)`, the `0x00C0` is gone, and
`0x0078` is still absent.

**The correlation was an artifact of the window, and widening it destroys it.** Over every
archived login log rather than the newest fifteen: `[L]`

```text
login-20260819-220418 .. login-20260821-*   token=0  0x00C0=1  0x0078=1  0x0011=1
```

Dozens of runs from 2026-08-19 to 2026-08-21 carried **no token**, sent **`0x00C0`**, and
selected a character perfectly well - and they were direct-client runs, because the launcher
did not exist yet. So neither the missing token nor the auth-failure report blocks selection.

**And the instrument was worse than wrong, it was ambiguous.** `0x0078 = 0` does not mean the
client refused; it also means **nobody clicked a character**. `login-20260901-015404` carries a
token *and* has `0x0078 = 0`. Most of the zero rows are short probe runs. The count I built the
finding on cannot distinguish a refusal from an idle character screen, and I did not check that
before reporting it as 10 for 10.

This is `CLAUDE.md`'s *"the filter was a timestamp"* rule, and its *"a summary line is not a
check"* rule, in one mistake. The fix was there for the cost of one wider `ls`.

### 5.3 What is actually known about the direct path

| | create=on | multiclient | token | full probe | selected? |
|---|---|---|---|---|---|
| 2026-08-19 .. 08-21, many runs | n/a | n/a | no | yes | **yes** |
| 2026-09-05 21:10 | yes | yes (stale marker) | no | yes | **no** |
| 2026-09-06 23:27 | no | no | no | minimal | **no** |
| 2026-09-06 23:31 | no | no | no | minimal | **no** |
| 2026-09-06 23:38 | no | no | **yes** | minimal | **no** |
| every launcher run | yes | yes | yes | yes | **yes** |

`[L]` for the rows. Direct runs **used to work** and stopped somewhere between 2026-08-21 and
2026-09-05; login enforcement landed 2026-09-05. The 2026-09-05 21:10 run had everything except
the token and failed, and attempt 3 had the token and nothing else and failed, so **no single
one of these four is sufficient on its own** and the combination has not been tried. `[D]`

**I am not spending a fourth launch on it.** The direct path is a means, not the question, and
three launches have bought one retraction and one flag.

### 5.4 What to run instead, and it needs no new configuration

`start-servers.cmd` already passes `-SetFieldProbe -ServersOnly -PoolSentry`, so the ordinary
path arms the sentry: **double-click it, then `maplecw-launcher.exe`, sign in, Start Game.**
`[L]` With `-SentryDumps` now defaulting to 4 that is a full sentry run on the path that
demonstrably works.

It is **not** the patch control - it carries the launcher's whole patch set - so it does not
answer §0's question. What it does answer is §4, for free: **three catches settle whether the
damage lands beside the 30-second census**, and that is the only lead in this file that a
single ordinary run can turn into a finding.

Running the patch control at all needs the launcher to stop writing the probe and session
markers with its own defaults. That is a change to `crates/launcher/src/client.rs`, not a
command line, and it should be made deliberately rather than worked around a fourth time.

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
