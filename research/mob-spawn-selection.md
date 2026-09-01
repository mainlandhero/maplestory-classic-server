# Which spawn points hold a mob, and why

2026-09-01. Subject: `world::config::choose_spawns`, which replaces `share_balanced`.

The owner, 2026-09-01: *"I'm starting to doubt myself that the mob cap is not split by ratio, but
rather randomly picked out across all of the mob spawns. The ratio of the mobs on the map is
less important than what we previously thought."*

Labels: **[L]** read straight out of the client's data or image. **[D]** derived by an
argument written down here. **[I]** inference or policy with no measurement behind it.

**No client run was needed for any of this, and none was made.**

---

## 0. The instruments, before their answers

Two instruments, structurally different, so they do not share a blind spot.

**A. A WZ census.** `wz-dump cat` over every map image, collecting the *whole key space* of
every `info` node and of every `life` entry with `type == "m"`. This **enumerates**; it does
not look up a list of candidate names. That distinction is the entire reason it is worth
running — the standing claim in `config.rs` was about map 40's `info` node alone, which is a
filter. Map 40 could simply have been a map carrying no override.

| control | result |
|---|---|
| field images read | **426 / 426**, 0 unreadable, 0 without an `info` node |
| `version`, `bgm`, `mapMark` (positive) | 426 / 426 |
| `fieldType` (discriminating positive) | **379** / 426 — present on some, absent on 47 |
| `life` entries of `type == "m"` | **9928**, matching `gm-handbook/mobs.txt`'s own count |

`fieldType` is the one that matters: a reader returning a constant set would score 426 on it.
It does not. [L]

**B. A byte scan of `client-patched\MapleStory.exe`** (73.1 MB) for each key name, ASCII and
UTF-16LE. This asks a different question — not *what does the data carry* but *does the client
ever look the name up*.

| | ascii | utf16 |
|---|---|---|
| `fieldType`, `foothold`, `portal`, `returnMap`, `town` (positive) | 2, 4, 4, 1, 9 | 2, 4, 6, 3, 5 |
| `notAKeyAtAll`, `capacityZZ`, `returnMapZZ` (negative) | 0 | 0 |

It discriminates in both directions. [L]

> **Its blind spot, stated so it can be quoted when acting on a zero:** a key name assembled
> at runtime from pieces, stored compressed, or reached by a hash rather than a literal would
> not appear. **This scan can prove presence, never absence.** That is why the negative below
> is carried by the WZ enumeration, and the scan is only corroboration.

---

## 1. Nothing in this client can decide which spawn points hold a mob

That sentence, in those words, is the result.

**1.1 There is no capacity field, on any map.** [L] The 426-field enumeration yields **57
distinct `info` keys**. In full, by frequency:

```
426  bgm cloud forcedReturn mapMark mobRate returnMap town version
424  hideMinimap
413  VRBottom VRLeft VRRight VRTop fieldLimit
380  swim            379  fieldType
375  AmbientBGM AmbientBGMv fieldScript fly noMapCmd onFirstUserEnter onUserEnter
     partyStandAlone quarterView standAlone
374  fieldLimit2     344  MRBottom MRLeft MRRight      343  MRTop
283  personalShop    260  mapDesc    219  fieldLimit_tw    187  moveLimit
 24  fs              21  decHP limitSpeedAndJump        11  hiredMerchant
  6  genReturnMap syncEffectObjectOnEnter               3  miniMapOnOff timeLimit
  2  recovery
  1  LBBottom blockScriptItem limitUI limitUseShop limitUseTrunk limitUserEffectField
     noChair noHekatonEffect offSoulAbsorption reviveCurField vanishAndroid vanishDragon
     vanishPet
```

No capacity, cap, quota, share, max or count field of any name. The only spawn-related key is
**`mobRate`**, present 426/426 as a float (`1.0000002` on map 40, values from `0.0` upward
across the set) — and a rate is not a cap.

**1.2 There is no per-spawn-point weight either.** [L] Every key on all 9928 `life` entries of
type `m`:

```
9928  id type x y cy fh rx0 rx1
8960  f          7414  mobTime      1840  hide
1026  forcedZMass forcedZPage       14  limitedname       1  useDay  useNight
```

No weight, no priority, no group, no share. `mobTime` is the per-point respawn delay the
server already reads.

`hide` looked like a lead and is not one, which is worth a line so nobody spends a pass on
it. It is on 1840 points across 89 maps and `tools/dump_portals.py` does not emit it, so it
never reaches the server — but **every one of the 1840 values is `0`**. [L] The flag is
present and never set, so the server ignoring it costs nothing. Reported because "1840 hidden
spawn points the server throws away" is exactly the confident wrong claim that comes from
counting a key's occurrences without reading its values.

**1.3 The client does not read even the key that exists.** [L] `mobRate` scans **0 ASCII, 0
UTF-16LE**. So does `mobTime`. The client parses `returnMap`, `forcedReturn`, `fieldType` and
`foothold` — the byte scan finds all of those — and neither of the two spawn keys.

**1.4 The capacity-shaped names that DO appear are something else**, and they are reported
rather than waved away, because a candidate that comes back non-zero and is passed over is
exactly how a clean confident zero gets written down. [L]

| name | hits | what it actually is |
|---|---|---|
| `maxMobCount` | 0 / 1 | in a run with `hitLimitEveryMob`, `noHitWhenMaxCount`, `totalHitCount`, `mobTemplateID`, `rush` — a **skill attack** property block: how many mobs one swing may hit |
| `mobCount` | 4 / 2 | four separate skill tables: `mobCountForDamagedByMob`; `mobCount`/`minAttackableCount`/`attackExceptTarget`; `#mobCount` in the `#`-token string table; `#mobCountDamR` |
| `mobCap`, `mobMax`, `capacity`, `spawnRate`, `mobLimit`, `spawnCount`, `mobGenRate`, `mobCapacity`, `maxMobCount` (ascii) | 0 / 0 | — |

`maxMob` and `maxMobCount` return the same single UTF-16 hit: `maxMob` is a prefix of it.

**1.5 And underneath all of it, the client cannot spawn a mob at all.** [L] Established
previously and re-read here rather than assumed: the field loader walks `life` **only to
preload `Mob/%07d.img` art**, reading `type`, `id`, `x`, `y`, `cy`, `fh` and nothing further,
and a populated mob is only ever built from a packet. `research/npc-spawn.md` §2,
`research/mob-spawn.md`.

**So the cap, and the choice of which points it fills, are server policy.** [D] There is no
measurement that could settle them, because there is no client behaviour to measure against.
This is a design decision, and it is the owner's.

That is a real result and not a shrug: it means the correct question is no longer *"what did
the real server do"* but *"what do we want"*, and the two halves should stop being argued as
if evidence could arrive.

---

## 2. What changed

`share_balanced` grouped the spawn points by template, gave each type
`floor(count * cap / total)` slots, handed leftovers out by largest remainder (Hamilton) with
ties broken by template id, and then shuffled *within* each group.

`choose_spawns` draws a **uniform random sample of `cap` points from the whole set** — a
partial Fisher–Yates over all indices, first `cap` kept, sorted back into spawn order. It has
no notion of a template at all.

`spawn_capacity` is **unchanged**. Nothing found here bears on it: the 75%/100% split and the
floor are still [I] from the same unofficial fan site, the single datapoint is still
40 points → 30, and floor and ceiling of `3n/4` still both reproduce it, so the rounding is
still unsettled. The one thing that did change is its doc comment, which claimed less than
what is now measured — it cited map 40 alone where §1 enumerates all 426.

The old name is kept as a one-line forwarding shim, because the single call site is in
`crates/world/src/fields.rs`, which belongs to another agent this session. **It should be
deleted** once `Fields::seed` points at `choose_spawns`.

---

## 3. What this does to the ratio — and it is arithmetic, not opinion

**Uniform sampling does not throw the ratio away.** Drawing `N` of `M` points uniformly
without replacement makes the count of any type with `K` points **hypergeometric**:

```
E[X]   = N * K / M                                   <- EXACTLY the type's share of the map
Var[X] = N * (K/M) * (1 - K/M) * (M - N) / (M - 1)
                                    ^^^^^^^^^^^^^ finite-population correction
```

So each type's expected share equals its share of the map: the ratio is **preserved in
expectation without being enforced**. [D] The draw cannot favour a type it cannot see.

### The worked example, kept from the old doc so the two compare directly

> **The map's NAME was wrong for two weeks, and the numbers were always right.** Both the
> original `config.rs` doc of 2026-08-19 and this pass called this map *"The Field South of
> Ellinia"*. It is not: that is map **10002010**, which has **66** spawn points across **four**
> types — Stump 27, Slime 18, Dark Stump 13, Green Mushroom 8. The 45-across-five profile
> `16 / 10 / 7 / 6 / 6` exists on exactly **one** map in this client, and it is **1006**.
> Checked by enumerating all 289 maps that have spawns rather than by looking the name up.
>
> Nothing downstream moves: every number in this section was computed from the profile, not
> from the name, and the profile was correct. It is corrected because a label that names the
> wrong map sends the next person to a map where none of this reproduces — and this repo
> treats a stale claim as a bug.

**Map 1006, "Hunting Ground Middle of the Forest II"** — 45 spawn points across five types,
`spawn_capacity(45, 1)`
= 33. All figures exact rational arithmetic; the pmf sums to 1 and its mean reproduces the
closed form, which is the check on the algebra.

| type | points | share | E[count] | SD | possible | middle 90% | P(absent) | old rule |
|---|---|---|---|---|---|---|---|---|
| Snail | 10 | 22.2% | **7.33** | 1.25 | 0..10 | 5..9 | 2.1e-08 | 7 |
| Blue Snail | 16 | 35.6% | **11.73** | 1.44 | 4..16 | 9..14 | impossible | 12 |
| Shroom | 7 | 15.6% | **5.13** | 1.09 | 0..7 | 3..7 | 1.8e-05 | 5 |
| Red Snail | 6 | 13.3% | **4.40** | 1.02 | 0..6 | 3..6 | 1.1e-04 | **5** |
| Orange Mushroom | 6 | 13.3% | **4.40** | 1.02 | 0..6 | 3..6 | 1.1e-04 | **4** |
| | 45 | 100% | **33.00** | | | | | 33 |

**How far a given spawn can drift:** about **one slot** either side of a type's share. The
middle 90% of draws keeps every type within roughly ±2. `P(some type is missing entirely)` is
at most **2.4e-4** — about **one draw in 4092** — and Blue Snail literally cannot be absent,
since 33 points drawn from 45 must include at least 4 of any 16.

**Why the spread is that small**, which is the part worth internalising: the cap is 75%, so
only **12 of 45** points are left out. The finite-population correction `(M-N)/(M-1) = 3/11`
**divides the variance by 3.7**. A uniform draw at a 75% cap simply has very little room to
be lopsided. If the cap were ever loosened, the drift would grow.

### The argument the other way, which is the one that decides it

Look at the last column. **Red Snail and Orange Mushroom hold the same 6 points of 45 — the
identical share** — and largest-remainder handed them **5 and 4**. Not on one spawn: on
*every* spawn, for the life of the process, because the two tied on remainder 18 and the tie
broke on template id.

Enforcing the ratio is what made two equal types permanently unequal. Sampling gives both
4.40. [D]

So the owner's instinct is well-founded in a way that is stronger than "it doesn't matter much":
the enforced version was not more faithful to the ratio, it was *less* — it converted a
rounding tie into a permanent bias, and it did so silently.

---

## 4. Does a respawn go back to its own point, or to a newly chosen one?

**Decision: back to its own point.** No change to the respawn path. [I], policy.

The existing path is keyed by `(map, object_id)`: `Fields::seed` books each chosen point into
`pending`, `Fields::hurt` re-books *the point that died*, and `due_respawns` rebuilds the mob
at that point's stored `x`/`cy`/`fh`. Keeping it costs **zero lines**.

The alternative — a death frees its point and a fresh point is drawn from the dormant pool —
costs the following, all of it in `fields.rs`:

1. **`mobTime` stops meaning what it means.** `mob_respawn_s` is keyed per point. If the new
   point is chosen at death time you use *its* delay for a death that happened elsewhere; if
   at fire time, the delay cannot be known when the timer is set. Either way the WZ's
   per-point value becomes a per-map value in effect.
2. **`MOB_TIME_NEVER` is silently defeated.** Exactly one spawn point in this client says
   `-1`, never respawn. Today it is simply never booked. Under a moving scheme it becomes a
   legal *destination* for some other point's timer, resurrecting the one point the data says
   must stay empty — unless an explicit exclusion is added and remembered.
3. **Mobs would migrate across the map over a session.** A snail killed on the left reappears
   anywhere. Nobody asked for that, and the owner's earlier sentence on this scoped it to the
   opposite: *"the spawn points that gets activated should be randomly chosen even on fresh
   spawn."* Fresh spawn is what they named.

What it would buy is re-randomisation over time, so a type that drew low at seed recovers.
Against §3's numbers that is worth very little: the drift is ~1 slot and total absence is
1-in-4092. **The smaller change is also the better one here**, so it is taken.

### The one thing this costs, stated plainly

`Fields::seed` sets `seeded = true` and never clears it, so **the draw happens once per map
per channel process** and the chosen set is frozen until restart. The spread in §3 is
therefore the spread of *one draw that lasts a whole session*, not something that averages out
over an evening. [D] Under the old rule the type mix was a constant; it is now a random
variable fixed at first entry. That is a genuine behaviour change and is the honest cost of
the decision above — if it ever proves annoying in play, the cheap fix is a periodic re-draw
at seed level, not a per-respawn one, and trap (2) is the thing to watch.

---

## 5. Determinism

`splitmix64` stays, and so does the seed threaded from `Fields::seed`
(`(map << 32) ^ now_ms * 0x9E3779B9`). Same seed, same field, exactly; different seeds,
different fields. Every distributional test below pins its seeds, so all of them are
deterministic — they pass or fail identically on every run, and none is flaky.

The modulo in `% (total - i)` is biased by about `total / 2^64` ≈ **2e-18** at `total = 45`,
against the 1e-4 probabilities in §3. Named in the code rather than ignored, and far too small
to justify rejection sampling.

---

## 6. What the tests now say

Four assertions stated the deleted rule and were rewritten. **Each rewrite was checked by
putting the old largest-remainder body back behind the new name and re-running:** the four
rewritten tests fail, and the sixteen untouched ones still pass. A rewritten test that still
passes against the rule it replaced was weakened, and that is the check that catches it.

| was | now | why this is the rule changing |
|---|---|---|
| `shuffling_positions_does_not_disturb_the_per_type_quota` — pinned counts to exactly 7/12/5/5/4 on four seeds | `the_per_type_count_is_no_longer_pinned` — every type must take **more than one** distinct count over 200 seeds, and the old fixed split must not be a majority of draws | The pinned map *was* the Hamilton rule written down. Replaced by a strictly stronger claim: the old code returns an identical map for every seed, so it fails on the first assertion for all five types |
| `a_mixed_map_keeps_each_types_share_rather_than_the_first_n` — asserted the exact split, then that every type was `< 1.0` slot from its proportion | `each_types_expected_share_is_its_share_of_the_map` — over 4000 draws each type's **mean** within 0.15 of `cap*K/M`, plus equal shares must draw equally | The `< 1.0` bound is now simply false per draw: Blue Snail's SD alone is 1.44. The band replacing it is **6 standard errors** (SD/√4000 ≤ 0.0227), derived not tuned; the code's worst error is 0.029 and the old rule misses by **0.60** on Red Snail |
| — | `the_spread_around_that_share_matches_the_hypergeometric` (new) | The SD table in §3 came from algebra, and a number from algebra is a claim like one read off a header. Asserts observed SD within 10% of the closed form; measured worst 1.4%. Also the only test that catches a draw that hits the right mean with collapsed variance — the old rule's SD was exactly 0 |
| `the_edges_do_not_panic_or_overshoot` — `one[0].template_id == 1, "the largest share takes the only slot"` | `a_cap_of_one_goes_to_a_random_point_not_the_largest_type` — over 2000 draws both types win, at 0.6/0.4 within 0.05 | That assertion was the apportionment showing through at the smallest cap. Pinning the *frequency* rather than settling for "one of the two" is what distinguishes a uniform draw from a biased one; the old rule scores 2000/0 |

Kept unchanged, and still passing: `a_fresh_spawn_spreads_across_the_map_instead_of_the_first_n`
(the low-x bunching fix — structurally impossible now rather than merely fixed),
`different_seeds_choose_different_spawn_points`, `map_40_keeps_thirty_of_its_forty_spawn_points`,
`the_chosen_mobs_come_back_in_spawn_order`, the remaining edge cases,
`a_small_map_shows_the_rounding_that_is_still_unsettled` and
`a_crowded_field_fills_every_spawn_point`. The naive-prefix check — that the first 33 in WZ
order miss a whole type — is retained as the bug still being ruled out.

`cargo test -p world --lib`: **748 passed, 0 failed**, including every `fields.rs` test through
the compatibility shim.

---

## 7. Handover

* **Delete the `share_balanced` shim.** Point `fields.rs:227` (`Fields::seed`) at
  `choose_spawns` and remove it. A second name for one behaviour is how a stale one survives.
* **`spawn_capacity`'s rounding is still unsettled** and is untouched by this work. 40 → 30
  remains the only datapoint and both roundings reproduce it. `spawn_capacity(6, 1)` = 4 where
  rounding up gives 5, and that test pins it so a change is deliberate.
* **The crowded branch is still written and untaken.** `players` is hard-coded 1 at the call
  site because there is no field-occupancy tracking; `Bus::others_on(id, map)` is the count it
  is waiting for.
* **Nothing here authenticates anybody**, as everywhere else in this server.
