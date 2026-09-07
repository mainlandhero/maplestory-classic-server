# Magician, first job: what this client's own data says every skill does

Written 2026-08-27, from `client-patched/Data/Skill/Skill_000.wz` and
`client-patched/Data/String/String_000.wz` via the new `tools/dump_skills.py`.
**No client run was spent on this, and no Ghidra pass was made.**

The owner: *"I want to verify that all Magician 1st job skills are working first."* Before this
file the repo had **no skill data at all**. `net::buff::buff_level` models exactly one skill
(Nimble Feet, 1002); every other cast is answered with *"This server does not grant skill N's
effect yet."* This is the data half of the gap, and only the data half.

Every claim is tagged **[L]** (read off this client's archive, its own tooltip text, or a
listing), **[D]** (derived from two or more [L] facts), **[I]** (inferred). There is a
["what I did NOT establish"](#7-what-i-did-not-establish) section and it is not short.

Regenerate the table with, from the repo root:

```
python tools/dump_skills.py
```

and pull the six rows out of it with:

```
python -c "print(''.join(l for l in open('gm-handbook/skills.txt',encoding='utf-8') if l.startswith('20')))"
```

---

## 0. The short version

| question | answer |
|---|---|
| How many first-job Magician skills does **this** build ship? | **Six.** [L] |
| Which ones? | Improved MP Recovery, Max MP Increase, Magic Guard, Magic Armor, Energy Bolt, Magic Claw |
| How many does this server model? | **Zero.** `buff_level` handles only skill 1002. [L] |
| Can the owner even put a point in one today? | **No.** `session::skills::on_skill_up` refuses every id not in `net::skills::BEGINNER_SKILLS` — its own comment says *"what a job may learn is `Skill.wz` data nobody has read"*. That is the data this file supplies. [L] |
| Is Magic Guard a timed buff? | **No — it is a toggle.** It carries no `time` at any level and its description says the effect is switched on and off by using the skill again. [L] |
| Do the two attacks share a formula with the physical one? | No. They carry `mad`, never `damage`, and **no skill in the whole archive carries both**. [L] |
| What is `mastery`? | A **level 1..10**, not a percentage — the client's own tooltip says "Mastery level 3". [L] |
| Biggest unit trap here | `x` and `y`. They mean a different thing on every skill and the WZ never says which. [L] |

---

## 1. The enumeration, and why it is complete rather than remembered

`Skill_000.wz` holds one image per **skill book**. String.wz names book `200`
**"Introduction to Magic"** [L], which is what identifies job 200 without assuming a
numbering scheme. `200.img/skill` has exactly six children:

```
2000000  2000001  2001000  2001001  2001002  2001003
```

**[L]**, and the enumeration was done the way `CLAUDE.md` demands — enumerate before you
filter. Every one of the 33 images in the archive was dumped and every skill id in all of
them collected (182 skills). Exactly six have an id beginning `200`, and all six are in
`200.img`; nothing with a job-200 id hides in `ItemSkill.img`, `Attacktype.img` or the six
`92xxxxx` pseudo-skill images. **[L]**

This build has **no pirate branch**: the job books present are 000/1xx/2xx/3xx/4xx only. So
the skill list is this client's, not another version's.

### `maxLevel` is measured, not assumed

There is no `maxLevel` property on any of these six. The max level is the **number of `level`
children** — and that is not a convention taken on trust. `String.wz` states a
`[Master Level: N]` in each skill's own description, authored separately from the numeric
data, and across the whole archive **163 skills state one and 163 agree; 0 disagree**. **[L]**
`tools/dump_skills.py` prints that census on every run, so a repack that breaks it is loud.

The WZ property literally called `masterLevel` exists on only **three** skills in the entire
archive — the beginner ones — so reading it as "the max level" would have come back empty for
every skill here. **[L]**

---

## 2. The instrument, and the control that matters

`tools/dump_skills.py` refuses to write anything until four positive controls parse. The load-
bearing one is **Nimble Feet**:

| | WZ `0001002` | `crates/net/src/buff.rs` |
|---|---|---|
| level 1 | `mpCon 4, time 10, speed 10, cooltime 180` | `mp_cost 4, seconds 10, value 10, cooldown 180` |
| level 2 | `mpCon 7, time 20, speed 10, cooltime 180` | `mp_cost 7, seconds 20, …` |
| level 3 | `mpCon 10, time 30, speed 10, cooltime 180` | `mp_cost 10, seconds 30, …` |

Those repo numbers are the ones confirmed on a client. The extraction reproduces them exactly
**[L]**, and the client's own tooltip says the same thing in words —
*"MP -4; speed +10 for 10 sec. Cooldown: 3 min."* **[L]** Three independent statements of the
same four numbers.

Two more controls exist because the corresponding bug was hit while writing the tool:

* **String.wz's key is the ZERO-PADDED id.** `0001002`, not `1002`. A lookup by `str(int(id))`
  returns nothing for every beginner skill *and says nothing about it* — a blank name column
  is invisible. The first census run reported "160 agree, 16 state none"; with the key fixed
  it is 163/13. The tool now asserts the name rather than hoping.
* **Energy Bolt's reach is in `common`, not in `level`.** `common/range = "350"` and there is
  no per-level `range` on it at all. A reader of `level/<n>` alone loses it silently, so the
  tool merges `common` into every level row, names the merged columns in that row's
  `fromCommon` cell, and asserts the control.

One more trap the tool reports rather than absorbs: **1 499 numeric cells in this archive are
quoted strings, not integers** — `mpCon` on 61 level nodes, `x` on 100, `mastery` on one. A
reader that kept only `int` would come back short, clean and confident.

---

## 3. The passives — and they may need no packet at all

### 2000000 Improved MP Recovery — max level 15, `type 50`, no `psd`

| lv | x | y |
|---:|---:|---:|
| 1 | 1 | 5 |
| 2 | 1 | 6 |
| 3 | 1 | 7 |
| 4 | 1 | 8 |
| 5 | 1 | 9 |
| 6 | 1 | 10 |
| 7 | 1 | 11 |
| 8 | 1 | 12 |
| 9 | 1 | 13 |
| 10 | 1 | 14 |
| 11 | 1 | 15 |
| 12 | 1 | 16 |
| 13 | 1 | 17 |
| 14 | 1 | 18 |
| 15 | 1 | 20 |

Tooltip, level 15: *"Regenerates 1% of Max MP every 10 seconds; increases MP recovery from
items by 20%."* **[L]**

So `x` = **percent of Max MP per tick**, `y` = **percent bonus to MP restored by items**. Both
percentages. **[L] from the tooltip.**

> **The 10-second interval is NOT in the WZ.** It appears only in the tooltip text. Nothing in
> `Skill.wz` carries the regen period, so a server that ticks this has to get the period from
> somewhere else. **[L]** (an absence, and the search that establishes it is an exhaustive
> column census of every level property in the archive, not a grep.)

### 2000001 Max MP Increase — max level 15, `type 50`, **`psd = 1`**, requires 2000000 level 3

| lv | mmpR |
|---:|---:|
| 1 | 10 |
| 2 | 11 |
| 3 | 12 |
| 4 | 13 |
| 5 | 14 |
| 6 | 15 |
| 7 | 16 |
| 8 | 17 |
| 9 | 18 |
| 10 | 19 |
| 11 | 20 |
| 12 | 21 |
| 13 | 22 |
| 14 | 23 |
| 15 | 25 |

Tooltip: *"Max MP +10%"* … *"Max MP +25%"*. `mmpR` is a **percentage of maximum MP**, not an
amount. **[L]** This is precisely the shape of the three unit bugs `CLAUDE.md` records: an
`R` suffix means *ratio*, and 25 sent as a flat MP bonus would be invisible on screen.

### Do these need a packet?

**Not established, and the honest answer has two halves.**

* The client *can* read these properties. The client image contains `mmpR`, `mpCon`,
  `cooltime`, `mad`, `mastery`, `indiePdd`, `indieMdd` and `bulletCount` as UTF-16 string
  literals, and `research/skills.md` records that the skill window the owner opened was drawn from
  the client's own `Skill.wz`. **[L]** So the client has both the data and the property names.
* That is **not** proof it applies them. A string being present is not a read. **[D]**

What *is* settled is where the authority lives for the value these two change:
`research/user-hit.md` establishes three ways that **the client computes damage and does not
apply it** — the server owns HP and MP and sends them back. **[L]** So Max MP on screen is a
number this server sends. Either the server folds `mmpR` into `max_mp`, or the client adds it
locally on top of what the server sent, and those two are **distinguishable** — see §8.

---

## 4. The buffs — one toggle, one timed

### 2001000 Magic Guard — max level 15, `type 10`, **`processtype 113`**, no prerequisite

| lv | mpCon | x |
|---:|---:|---:|
| 1 | 8 | 30 |
| 2 | 8 | 33 |
| 3 | 8 | 36 |
| 4 | 8 | 39 |
| 5 | 8 | 42 |
| 6 | 10 | 49 |
| 7 | 10 | 52 |
| 8 | 10 | 55 |
| 9 | 10 | 58 |
| 10 | 10 | 61 |
| 11 | 12 | 68 |
| 12 | 12 | 71 |
| 13 | 12 | 74 |
| 14 | 12 | 77 |
| 15 | 12 | 80 |

Tooltip: *"MP -8; Replace 30% of HP damage with MP while active."* So `x` is a **percentage of
incoming damage redirected to MP**. **[L]**

> **It has no duration, and that is the data rather than an omission.** Magic Guard carries no
> `time` at any of its 15 levels. **[L]** That is not a one-skill oddity: **all 16 skills in
> this archive with `processtype 113` have no `time` at any level, and every one of their
> descriptions says the effect is switched on and off by using the skill again** — Magic
> Guard, Dark Sight, Combo Attack, Meso Guard, both Element Amplifications, every Final
> Attack. And of the 20 skills with `info/type == 10`, exactly two carry no `time`: Magic
> Guard and Dark Sight, both `processtype 113`. **[L]** for the census; **[D]** for
> "`processtype 113` means toggle", which is a correlation over 16 cases with no
> counterexample, not a decoded field.
>
> The consequence for the server is concrete: **`buff_level`'s `seconds` has no value to take
> here.** A toggle needs an off-path, and the one the client already uses is `0x013F`, which
> `crates/world/src/session/buff.rs` handles — *"removal is the server's job on both paths"*.

### 2001001 Magic Armor — max level 20, `type 10`, `processtype 6`, requires 2001000 level 3

| lv | mpCon | time | indiePdd | indieMdd |
|---:|---:|---:|---:|---:|
| 1 | 8 | 300 | 40 | 40 |
| 2 | 8 | 315 | 44 | 44 |
| 3 | 8 | 330 | 48 | 48 |
| 4 | 8 | 345 | 52 | 52 |
| 5 | 8 | 360 | 56 | 56 |
| 6 | 10 | 375 | 60 | 60 |
| 7 | 10 | 390 | 64 | 64 |
| 8 | 10 | 405 | 68 | 68 |
| 9 | 10 | 420 | 72 | 72 |
| 10 | 10 | 435 | 76 | 76 |
| 11 | 13 | 450 | 80 | 80 |
| 12 | 13 | 465 | 84 | 84 |
| 13 | 13 | 480 | 88 | 88 |
| 14 | 13 | 495 | 92 | 92 |
| 15 | 13 | 510 | 96 | 96 |
| 16 | 16 | 525 | 100 | 100 |
| 17 | 16 | 540 | 104 | 104 |
| 18 | 16 | 555 | 108 | 108 |
| 19 | 16 | 570 | 112 | 112 |
| 20 | 16 | 600 | 120 | 120 |

Tooltip: *"MP -8; Weapon Def. +40, Magic Def. +40 for 300 sec."* **[L]** So:

* `indiePdd` = **flat Weapon Defence points**, `indieMdd` = **flat Magic Defence points**.
  Not percentages — the tooltip writes `+40`, and the `R`-suffixed ratio forms
  (`indiePddR`, `indieMhpR`) exist elsewhere in this same archive as separate properties, so
  the distinction is the data's own. **[L]**
* `time` is **SECONDS**. Same unit as Nimble Feet's, which the repo has confirmed on a client.
  **`net::buff::TemporaryStat::duration_ms` is MILLISECONDS** — multiply by 1000. Level 1 is
  `300` here and must go on the wire as `300000`. Getting that wrong puts the expiry 300 ms
  out and the icon flashes and vanishes, which is the failure mode `crates/net/src/buff.rs`
  already documents. **[L]**
* Magic Armor is the **only** skill in the archive carrying `indieMdd`; its 20 level nodes are
  all 20 occurrences. **[L]**

`indieMdd` and `indiePdd` are equal at every level, but they are **two properties** and the
server should carry both — a future skill separating them would silently inherit the wrong
one otherwise.

Each of these two buffs needs **one `TemporaryStat` bit**. Which bits — the 124-byte mask
positions the way `CTS_SPEED = 92` was established — is the other agent's job and nothing here
guesses at it. What this file fixes is the **payload**: Magic Guard grants one stat whose value
is `x` (a percent, 30..80) and no duration; Magic Armor grants **two** stats, weapon defence
and magic defence, value 40..120 each, duration `time * 1000` ms.

---

## 5. The attacks — the data only, the formula deliberately not

`research/damage-formula.md` says at its wand section: *"Magic damage is a different path
entirely and is not in this function."* Another agent is on that. What follows is the input,
not the arithmetic.

The archive backs that separation up on its own terms: **47 skills carry `damage`, 14 carry
`mad`, and zero carry both.** All 14 `mad` skills are magician skills. **[L]**

### 2001002 Energy Bolt — max level 20, `type 2`, `processtype 116`, no prerequisite

`common`: `bulletCount = "1"`, **`range = "350"`** — level-invariant, and the only statement of
this skill's reach anywhere in its node.

| lv | mpCon | mad | mastery | attackCount | mobCount |
|---:|---:|---:|---:|---:|---:|
| 1 | 8 | 90 | 1 | 1 | 1 |
| 2 | 8 | 92 | 1 | 1 | 1 |
| 3 | 8 | 94 | 1 | 1 | 1 |
| 4 | 8 | 96 | 2 | 1 | 1 |
| 5 | 9 | 98 | 2 | 1 | 1 |
| 6 | 9 | 100 | 3 | 1 | 1 |
| 7 | 9 | 102 | 3 | 1 | 1 |
| 8 | 10 | 104 | 4 | 1 | 1 |
| 9 | 10 | 106 | 4 | 1 | 1 |
| 10 | 11 | 108 | 5 | 1 | 1 |
| 11 | 11 | 110 | 5 | 1 | 1 |
| 12 | 12 | 112 | 6 | 1 | 1 |
| 13 | 12 | 114 | 6 | 1 | 1 |
| 14 | 13 | 116 | 7 | 1 | 1 |
| 15 | 13 | 118 | 7 | 1 | 1 |
| 16 | 14 | 120 | 8 | 1 | 1 |
| 17 | 14 | 122 | 8 | 1 | 1 |
| 18 | 15 | 124 | 9 | 1 | 1 |
| 19 | 15 | 126 | 9 | 1 | 1 |
| 20 | 16 | 130 | 10 | 1 | 1 |

### 2001003 Magic Claw — max level 20, `type 1` + **`magicDamage 1`**, `processtype 2`, requires 2001002 level 1

**No `range`, no `lt`/`rb`, no `bulletCount`, no `common` node at all.** **[L]**

| lv | mpCon | mad | mastery | attackCount | mobCount |
|---:|---:|---:|---:|---:|---:|
| 1 | 10 | 45 | 1 | 2 | 1 |
| 2 | 10 | 46 | 1 | 2 | 1 |
| 3 | 10 | 47 | 1 | 2 | 1 |
| 4 | 10 | 48 | 2 | 2 | 1 |
| 5 | 11 | 49 | 2 | 2 | 1 |
| 6 | 11 | 50 | 3 | 2 | 1 |
| 7 | 11 | 51 | 3 | 2 | 1 |
| 8 | 12 | 52 | 4 | 2 | 1 |
| 9 | 12 | 53 | 4 | 2 | 1 |
| 10 | 13 | 54 | 5 | 2 | 1 |
| 11 | 13 | 55 | 5 | 2 | 1 |
| 12 | 14 | 56 | 6 | 2 | 1 |
| 13 | 14 | 57 | 6 | 2 | 1 |
| 14 | 15 | 58 | 7 | 2 | 1 |
| 15 | 15 | 59 | 7 | 2 | 1 |
| 16 | 16 | 60 | 8 | 2 | 1 |
| 17 | 17 | 61 | 8 | 2 | 1 |
| 18 | 18 | 62 | 9 | 2 | 1 |
| 19 | 19 | 63 | 9 | 2 | 1 |
| 20 | 20 | 65 | 10 | 2 | 1 |

Three things about these two that are easy to get wrong:

* **`attackCount` is attacks per cast, and Magic Claw's is 2.** Tooltip: *"Use MP to attack an
  enemy twice."* **[L]** `mad 45` is therefore per hit, not per cast — a formula that applies
  45 once will produce half the damage and look like a formula bug rather than a count bug.
  `mobCount` is 1 on both: single target.
* **`mastery` is a LEVEL 1..10, NOT a percentage.** Tooltip: *"Mastery level 1"* … *"Mastery
  level 10"*, and the same 1..10 range with the same wording appears on every warrior weapon-
  mastery skill in this build (*"Sword Mastery level 1"*). **[L]** In other MapleStory versions
  this same property is a percentage in the 10..60 range; **it is not one here**, and feeding
  10 into a formula expecting a percent collapses the damage spread. **What a mastery level
  does to the spread is not in `Skill.wz` at all** — that mapping has to come from the client.
* **`mad` is the client's "Basic Attack" number.** Energy Bolt level 1 `mad 90` renders as
  *"Basic Attack 90"* — the tooltip carries no `%` sign. That it is a percentage multiplier is
  **[I]**, by analogy with `damage` on the physical skills; the tooltip does not say so and no
  formula has been decoded. Report the 90; do not assume the multiply.

Neither attack has `elemAttr`, so neither is elemental in this build's data. **[L]**
(21 skills in the archive do carry one.)

`info/magicDamage = 1` on Magic Claw is a **singleton — the only occurrence in all 182
skills.** **[L]** Meaning not established, and it is emitted raw rather than interpreted; a
one-of-one field is exactly the kind of row `CLAUDE.md` warns about writing from a quick read.

---

## 6. Every unit, and the evidence for each

The client states its own units in `String.wz/Skill.img/<id>/h<level>` — the line the player
reads — so `gm-handbook/skills.txt` carries that text as its last column and every unit below
is checked against it rather than assumed.

| property | unit | evidence |
|---|---|---|
| `mpCon` | flat MP points | Nimble Feet L1 `4` ↔ "MP -4" ↔ `buff.rs` `mp_cost 4`, confirmed on a client. **[L]** |
| `time` | **seconds** | Nimble Feet L1 `10` ↔ "for 10 sec"; the wire wants ms, so ×1000. **[L]** |
| `cooltime` | **seconds** | Nimble Feet `180` ↔ "Cooldown: 3 min." **[L]** (none of the six job-200 skills has one) |
| `indiePdd`, `indieMdd` | flat defence points | Magic Armor L1 `40` ↔ "Weapon Def. +40, Magic Def. +40". **[L]** |
| `mmpR` | **percent of Max MP** | Max MP Increase L1 `10` ↔ "Max MP +10%". **[L]** |
| `x` (Magic Guard) | percent of incoming damage | L1 `30` ↔ "Replace 30% of HP damage with MP". **[L]** |
| `x`,`y` (Improved MP Recovery) | percent of Max MP / percent item bonus | L1 `1`,`5` ↔ "1% of Max MP every 10 seconds; … items by 5%". **[L]** |
| `mad` | the tooltip's "Basic Attack" number | Energy Bolt L1 `90` ↔ "Basic Attack 90". Percent multiplier is **[I]**. |
| `attackCount` | attacks per cast | Magic Claw `2` ↔ "attack an enemy twice". **[L]** |
| `mobCount` | targets per cast | `1` on both attacks. **[L]** |
| `mastery` | **level 1..10** | "Mastery level N" on all eight skills carrying it. **[L]** |
| `range` | client pixels | Energy Bolt `350`; the archive's other `range` values are 130..180 melee reaches. **[D]** — no tooltip states a pixel count, so the unit is by comparison with skills whose reach is known to be short. |
| `bulletCount` | projectiles per cast | Energy Bolt `1`, from `common`. **[L]** |

**`x` and `y` deserve the warning twice.** They mean a different thing on every skill that
carries them — Magic Guard's `x` is a damage percentage, Improved MP Recovery's `x` is an MP
percentage, Flash Jump's `x` is a distance — and **nothing in the WZ says which**. The tooltip
column is the only thing in the generated table that does. Treat any `x`/`y` read without
reading its tooltip as unverified.

---

## 7. What I did NOT establish

* **Which `TemporaryStat` bit each buff uses.** Deliberately out of scope; another agent has
  the 124-byte mask.
* **Any magic damage formula.** `mad` and `mastery` are inputs to a function nobody here has
  decoded. `research/damage-formula.md` says magic is a different path and this file does not
  extend it.
* **What a `mastery` level of 1..10 does to the damage spread.** Not in `Skill.wz`. It has to
  come from the client, and I did not look — no Ghidra this session.
* **Whether the client applies the passives on its own.** The property names exist in the
  client image as UTF-16 literals, which means client code *can* look them up; that is not a
  read and I did not promote it to one. §8 has the cheap discriminator.
* **Magic Claw's reach.** It has no `range`, no `lt`/`rb` and no `common`. Either the client
  has a default for a non-projectile magic attack or the reach comes from the animation. Not
  established.
* **The regen period for Improved MP Recovery.** "every 10 seconds" is in the tooltip prose
  and in no numeric field of the archive.
* **What `type`, `processtype`, `skillType`, `additional_process` and `info/magicDamage`
  mean.** All emitted raw. The one correlation strong enough to write down is
  `processtype 113` ↔ toggle, and it is labelled **[D]** with its 16 cases named.
* **Whether Magic Guard's redirection happens client-side or server-side.** The client computes
  the damage number it reports in `0x00E5` (`research/user-hit.md`, [L]) and the server applies
  it. Whether the reported number is already net of Magic Guard is unknown and is a *packet*
  measurement, not a data one.
* **Whether any of this is reachable in-game today.** `!job 200` exists, but
  `session::skills::on_skill_up` refuses every non-beginner id, so the `+` button cannot put a
  point into any of the six. **[L]**

---

## 8. The cheap measurements, and what each outcome would mean

Neither of these needs a new packet decoded, and both are one variant at a time.

**A. Does the client apply `mmpR` itself?** Write skill `2000001` at level 15 into the store
for a job-200 character (`store::set_skill_level`), log in, and compare the Max MP drawn on
screen against the `max_mp` this server put in its own stat packet in `world.log`.

* identical → the client does **not** apply the passive; the server must fold +25% into
  `max_mp` itself, and until it does the skill does nothing at all;
* on screen is 1.25× the server's number → the client applies it locally, and a server that
  also applies it would **double-count**.

Both outcomes are actionable and they point opposite ways, which is what makes it worth a
launch.

> **ANSWERED 2026-09-06, on the HP twin, by the owner's screenshot.** Cobalt (job 100, level 18)
> with `1000001` Max HP Increase at 15: `maplecw.db` held `hp 358, max_hp 358`, `world.log`
> read *"idle regen +0 hp ... 358/358 hp - HP is full"*, and the screen drew **358 / 447**.
> `358 + ⌊358 × 25 / 100⌋ = 447` (447.5 would be 448, so the client **floors**). **The client
> applies it locally**, the second outcome - and the server, which had been comparing HP
> against its own 358, was calling them full and not regenerating. Fixed the same day:
> `world::itemrecovery::boosted_max` is the expression, `world::session::pools` applies it to
> every ceiling the server caps against (regen, potions, Recovery, level-up refill, `!heal`,
> quest set-HP, `!resetap`, the party bar), and the stat packet keeps sending the base. The
> MP twin is **[D]** by symmetry and has not been watched.

**B. Is `processtype 113` really a toggle on screen?** Once Magic Guard has a stat bit, cast it
twice. If the second cast turns the icon off rather than refusing or re-granting, the toggle
reading is confirmed and `buff_level`'s `seconds` field needs a "no duration" case rather than
a number.

Do **A** first: it costs nothing beyond a login, it does not depend on the mask work, and its
answer changes what "Max MP Increase works" even means.
