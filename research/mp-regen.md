# Improved MP Recovery's regen bonus — skill `2000000`'s `x` column

The **second** bonus on the skill whose first bonus was fixed on 2026-08-30. `y` is the item
percentage (`crates/world/src/itemrecovery.rs`, wired into `session/consume.rs`); **`x` is this
one**, and it lands on the idle regeneration tick in `crates/world/src/session/regen.rs`.

Labels are the project's: **[L]** read off this client's own data or off a capture, **[D]**
derived from two or more [L] facts, **[I]** inferred — a policy nothing on this machine can
confirm.

Status: **wired**, `crates/world/src/session/regen.rs`. Nine mutations were run against it and
every one is caught — §7.

---

## 1. The subject

`regen.rs` answered idle regeneration with a flat `REGEN_AMOUNT = 10` to both pools and
performed **no skill lookup at all**. The owner's original sentence (2026-08-20) was *"Whenever the
user is idle, they should get 10 HP and MP every 10 seconds"*, so the flat 10 was right when it
was written; what was missing is that one skill in this client says otherwise about MP.

This is the same shape as the three siblings already recorded in `research/item-recovery.md`
§1.2 — Magic Guard, Iron Body, and the item half of this very skill: *the bit or the level is
stored, and the arithmetic that makes it mean something is the server's*. A passive is the
worst case of it, because there is no icon to notice missing.

---

## 2. The columns, and the unit — which is the whole question

Read out of `gm-handbook/skills.txt` (`tools/dump_skills.py`, from the client's own `Skill.wz`
and `String.wz`). `x` is field 64, `y` is field 65, `tooltip` is field 98. All fifteen levels of
both skills, not a sample. **[L]**

```text
1000000  Improved HP Recovery  job 100  maxLevel 15  type 50
         x ABSENT at every level      y = 5..20
         "Regenerates HP every 10 seconds; increases HP recovery from items by N%"

2000000  Improved MP Recovery  job 200  maxLevel 15  type 50
         x = 1 at every level         y = 5..20
         "Regenerates 1% of Max MP every 10 seconds; increases MP recovery from items by N%"
```

### 2.1 The tooltip states the unit, in the client's own words

> *"Regenerates **1% of Max MP** every 10 seconds"*

**[L]** — verbatim from `String.wz`, byte-identical on all fifteen levels. One clause naming
the **pool** (Max MP), the **proportion** (`%`), and the **clock** (10 seconds). It is a
percentage of max MP, not a flat amount. The brief's premise is confirmed.

### 2.2 That the `1` is the `x` COLUMN is [D], not [L] — and here is why that is not pedantry

`dump_skills.py` reads the per-level h-string **verbatim**; it does not substitute. So for `y`,
which runs 5..20, the correspondence between column and text is observable fifteen times over
and is [L]. For `x`, which is `1` at every level, **nothing varies, so no correlation can be
measured** and a literal `1` baked into the string is indistinguishable from a substituted one.

The control is the **second-job twin** — the same skill name, the same ten-second clock, and an
`x` that *does* vary:

```text
1110000 / 1210000  Improved MP Recovery  (Fighter / Page)   x = 3..22 over 20 levels
   level  1   x=3    "Recover 3 additional MP every 10 sec."
   level 20   x=22   "Recover 22 additional MP every 10 sec."
```

Twenty levels, twenty agreements. **[L]** So `x` is what fills the recovery slot in an Improved
MP Recovery tooltip, and reading `2000000`'s `1` as its `x` is **[D]** on a near-subject
positive control rather than on a general "x is sometimes a percentage".

Enumerated across the whole book for the same reason: of 52 skills whose `x` takes three or
more distinct values, **23 render it into a `%` slot at every level and 18 render it as a flat
number at every level.** `x` genuinely carries both units, so the tooltip is the only thing
that can say which — exactly as `dump_skills.py`'s own header warns.

### 2.3 **The same column carries the other unit on the sibling skill**

This is the finding worth more than the fix, and it is `CLAUDE.md`'s *"the unit, not the
arithmetic"* with both readings live in one column of one book:

```text
1110000  x=22   "Recover 22 additional MP every 10 sec."     <- FLAT, and ADDITIONAL
2000000  x=1    "Regenerates 1% of Max MP every 10 seconds"  <- PERCENT of the pool
```

Same column, same skill name, same clock, two units. Reading `2000000`'s `x` as flat gives a
237-MP Magician **1** MP per tick where the client promises **2**; reading `1110000`'s as a
percentage would give a Fighter 22% of their pool per tick. Neither reading errors — both produce
a plausible number.

### 2.4 `1000000` has no `x`, so there is no HP regeneration number to wire

**[L]**, enumerated over all fifteen levels. The Warrior tooltip states no HP number either —
just *"Regenerates HP every 10 seconds"*. **A server that answers an HP regen amount here is
inventing it.** The symmetry with the `y` column, which *is* identical on both skills, is the
trap; `y` is symmetric and `x` is not.

HP therefore stays at the flat `REGEN_AMOUNT` unconditionally. This is asserted from both ends:
`itemrecovery::mp_regen_percent` refuses the HP skill id, and
`regen::tests::the_warrior_twin_changes_neither_pool` checks a level-15 Warrior regenerates
exactly what an unlearned character does, on **both** pools.

---

## 3. Composition: it **adds** to the flat base

Three readings were possible. **"Instead of" is eliminated by measurement**; the choice between
the other two is **[I]**, and it is defended on a screen that a player can see.

```text
Cobalt, the character the item half was reported on:   max MP 237
  the flat base                                        10 MP / tick
  1% of 237, floored                                    2 MP / tick
```

| reading | at 237 max MP | at 50 max MP | verdict |
|---|---|---|---|
| **instead of** the base | 2 | **0** | **[D] eliminated** |
| max(base, bonus) | 10 | 10 | never harms, but see below |
| **base + bonus** | **12** | **10** | **chosen** |

* **"Instead of" makes a learned passive a downgrade.** [D] from two [L]s: the flat base is 10,
  and 1% of every max MP this server has ever issued is below it. Worse, 1% of anything under
  100 **floors to zero**, and a fresh Magician's max MP is `5` — so "replace" means learning a
  recovery skill switches recovery off. That is not a taste argument.
* **Between "add" and "max above a floor"**, they are identical until max MP reaches 1000, which
  no character here has had. A max() reading therefore does *nothing observable on this server
  at any level* — indistinguishable on screen from the unwired state this change removes.
* **"Additional" is the client's own word for this bonus.** `1110000`'s tooltip says *"Recover N
  **additional** MP every 10 sec."* — the same skill family, on the same clock, composing
  additively with whatever base exists. **[L]** as a fact about `1110000`; applying it to
  `2000000` is an analogy and is the [I] here. `2000000`'s own tooltip does not use the word.

Rounding is **floor**, matching `itemrecovery::boosted`, `Restores::mp_for` and
`session::combat::on_user_hit`'s Magic Guard split. A fourth rounding rule in a fourth place is
how two of them end up disagreeing.

The floor at the bottom of the range is not hidden: when a learned skill contributes 0 because
the pool is too small, **the log line says so explicitly** rather than printing nothing, because
"nothing printed" is exactly what the unwired state looked like.

---

## 4. The tick interval — it agrees, and the agreement is thinner than it looks

`REGEN_EVERY_MS` is `10_000`; the tooltip says *"every 10 seconds"*. **They agree.**

Worth a section only because it would be easy to file as corroboration. **No column in
`Skill.wz` carries an interval for this skill.** Enumerated — every non-empty field on
`2000000` at every level is:

```text
skillId, job, level, maxLevel, name, type, hs, x, y
```

There is no `time`, no `subTime`, no `cooltime`, no `dotInterval`. **The tooltip text is the
single statement of the period anywhere in this client's data**, and this server already ticked
on ten seconds for an unrelated reason — the owner's sentence. Two things arriving at 10 from
different authorities is a coincidence that happens to be convenient, not a cross-check.

`the_tick_interval_is_the_one_the_tooltip_states` reads the generated file and asserts the
tooltip still says it, with a positive control (fifteen rows for `2000000`) first, because
`gm-handbook/` is gitignored and a check that silently skips looks exactly like one that passed.

`IDLE_AFTER_MS` — how long you must stand still before any of this begins — is a different
quantity and the tooltip says nothing about it. It stays the owner's rule.

---

## 5. Every other passive that scales a number `regen.rs` computes — enumerated

The question is *"what else puts a percentage on the HP or MP that the idle tick produces?"*
Asked of the whole book rather than of a candidate list, since `CLAUDE.md`'s two worst wrong
answers both came from searching a known set.

**Every skill in the archive whose tooltip names a repeating `every N sec` clock — all five:**

| skill | job | column | the client's own words | shape |
|---|---|---|---|---|
| `1000000` Improved HP Recovery | 100 | **no `x`** | "Regenerates HP every 10 seconds" | **no number exists** |
| `2000000` Improved MP Recovery | 200 | `x` = 1 | "Regenerates **1% of Max MP** every 10 seconds" | **PERCENT — this change** |
| `1110000` Improved MP Recovery | 111 | `x` 3..22 | "Recover N **additional** MP every 10 sec." | FLAT, second job |
| `1210000` Improved MP Recovery | 121 | `x` 3..22 | "Recover N **additional** MP every 10 sec." | FLAT, second job |
| `1311004` Dragon Blood | 131 | `x` 40 | "HP -40 every 3 sec" | self-damage, not regen |

> **Exactly one passive in this client puts a percentage on a number `regen.rs` computes, and
> it is `2000000`'s `x`.** **[L]**

`1110000` / `1210000` are the same shape on the same clock but **flat**, and they are job
111/121 — unreachable, since second-job advancement is itself unwired
(`crates/world/src/secondjob.rs`: *"THIS MODULE IS NOT WIRED"*). They are named here so that
whoever wires second job knows the tick already has a place for them, and knows their `x` is a
**different unit** from the one wired today.

A second sweep on the words *recover / regenerate / restore* returns 12 skills; the seven not in
the table above (`1001` Recovery, `2301001` Heal, `2301003` Bless, `2311002` Holy Symbol,
`4100002` Critical Recovery, `4111000` Meso Saver, `4200001` Nimble Recovery, `4210000` Chakra)
are all **event-triggered or cast**, not idle-clocked, and none of them scales the idle tick.
`session/recovery.rs` already implements `1001`'s shape.

### 5.1 Chairs: named by the owner, and there is no number to scale

The owner, 2026-08-21, named chairs in the same sentence as idle regeneration: *"that's only for idle
regeneration standing or sitting in a chair in the Set-up tab or sitting on a chair in a map."*

**This server has no chairs.** `crates/net/src/userpool.rs:486` sends the chair item id as a
literal `0` and `:501` writes *"no chair to decode"*; `session/consume.rs:139` records the
intent — *"Chairs are the other case the owner named and this server has no chairs yet"*. So there is
no chair recovery rate for a passive to scale, and nothing to wire. **[L]** from the code.

### 5.2 Equipment: no regeneration column exists

`gm-handbook/equips.txt` has 26 columns and **none of them is a recovery or regeneration
term** — `incMHP` / `incMMP` raise the *pools*, which changes what 1% means but is not itself a
regen modifier. So no item scales the idle tick either. **[L]**

---

## 6. What was changed

* `crates/world/src/itemrecovery.rs` — `MP_REGEN_PERCENT_OF_MAX` (the `x` column),
  `mp_regen_percent`, `mp_regen_percent_from_wz` (test-only, proves the table is the file's),
  `regen_of_max`. The column lives beside its `y` sibling so that **skill `2000000`'s WZ row is
  transcribed in one file and checked by one file-backed harness**.
* `crates/world/src/session/regen.rs` — `MpRegen`, `Session::mp_regen`, the MP half of the tick,
  and the log line.

The skill lookup sits **after** the early returns (not idle, already full, not due), so it costs
one row per ten seconds per idle session rather than one per loop.

`Store::skill_level` answers `Ok(0)` for an unraised skill, so `unwrap_or(0)` is reached only on
a real database failure — and there it degrades to the flat base rather than skipping the tick.
`CLAUDE.md`'s *always answer*: a regeneration that refuses is a bar that stops moving for the
rest of the session.

---

## 7. The tests, and the mutations that prove they discriminate

Nine injections, each applied alone to a clean tree, **every one caught**:

```text
1  pre-fix: MP uses the flat amount, no lookup   -> 2 failed
2  REPLACE the base instead of adding            -> 3 failed  (incl. the small-pool discriminator)
3  max-above-a-floor instead of adding           -> 2 failed
4  mirror the MP bonus onto HP as well           -> 2 failed
5  read x as FLAT, not a percent of max MP       -> 4 failed
6  the x table says 2 where the WZ says 1        -> 4 failed  (the file-backed test)
7  drop the Warrior guard in mp_regen_percent    -> 1 failed
8  tick every 5 s instead of 10                  -> 2 failed
9  invent an HP regen out of the y column        -> 1 failed
```

Injection 2 is the one that matters most: **at 237 max MP a "replace" reading still gives 2,
which is non-zero and looks like a working feature.** Only the 50-max-MP case separates it, and
`a_pool_too_small_to_earn_a_bonus_still_regenerates_the_flat_amount` is that case.

Injection 9 exists because `the_warrior_twin_changes_neither_pool` survived all eight of the
others — a test no mutation can break is pinning what the code does. The bug it actually guards
is the symmetry one (invent an HP regen from `1000000`'s `y`, which does exist), and it is the
only test that catches it.

Per `CLAUDE.md`'s Heena rule, `improved_mp_recovery_adds_one_percent_of_max_mp_to_the_mp_half`
asserts **four** effects of one tick — the database row, the `0x007C` body, the blue number
(which must *not* grow, since it is the HP amount), and the log line — rather than one.

### 7.1 The harness was broken first, and it answered cleanly

Worth recording because it is this repo's most-repeated failure, and it happened here:

> The first mutation harness ran `cargo test -p world --lib <dir> regen itemrecovery`. **`cargo
> test` accepts one testname.** It answered *"unexpected argument 'itemrecovery'"*, ran zero
> tests, and the `FAILED` regex found nothing — so **all eight injections came back "0 tests
> failed"** and every test looked like it was pinning the code.

The clue was `CLAUDE.md`'s own: *the result that does not vary is the clue* — eight mutually
contradictory injections cannot all be survivable. The harness now asserts `running N tests`
with `N > 700` before any silence is believed, which is the same rule as
`no_real_item_at_any_level_gets_a_bonus_that_rounds_to_nothing`'s `checked > 100`.

---

## 8. How the owner can see it, and what each outcome means

**No client run is needed to confirm the arithmetic.** A run only confirms the client draws the
server's number, and it rides on any session. `maplecw.db` already holds `(213, 2000000, 15)`,
so **no `!learn` is needed**.

> On Cobalt (max MP 237, `2000000` at level 15), spend some MP, then **stand perfectly still for
> ~25 seconds** — no moving, no attacking, no being hit; all three reset the countdown.
>
> * **MP rises by 12 per tick** — wired and correct. `world.log` carries
>   `[Improved MP Recovery: +2 MP on top of the flat 10, 1% of 237 max MP]`.
> * **MP rises by 10** — the bonus did not fire. The log line says which: absent entirely means
>   the skill level read as 0; *"floors to 0"* means the pool is smaller than the log claims.
> * **MP rises by 2** — the composition became a replacement rather than an addition. §3 says
>   this must not happen and injection 2 is the test that holds it.
> * **HP rises by anything other than 10** — an HP regen number was invented. §2.4 says this
>   client has none.
>
> The **blue number over the head is HP** and must stay `+10`; it is deliberately not the MP
> amount, and a `+12` there would mean the number and the bar disagree.

---

## 9. What this does NOT establish

* **Whether the client also applies this regeneration locally.** [D] says it cannot matter —
  `0x007C` carries the **absolute** MP total and the client draws what it is told — but that is
  the same reasoning recorded for Magic Guard and the item half, and it is not measured. An MP
  rise of `10 + 2 + 2` would falsify it.
* **Whether the base flat 10 is right at all.** It is the owner's number, not the client's; this
  client's data contains no base regeneration for anyone. Everything in §3 rests on it.
* **`1110000` / `1210000`.** Enumerated and deliberately not implemented — second job is unwired,
  and their `x` is the *other* unit.
* **Whether "additional" generalises from `1110000` to `2000000`.** That analogy is the [I] in
  §3. What is [D] rather than [I] is only the elimination of the replacement reading.
* **Chairs.** Named by the owner, no server support, no number to scale (§5.1).
