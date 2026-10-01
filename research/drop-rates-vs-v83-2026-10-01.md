# Drop chances: ours against the v83 server tables

The owner, 2026-10-01: *"I suspect that my drop logic even accounting for 5x droprate is too high.
Scrolls are dropping too often, and so are equipment."*

Labels: **[L]** read off a file or a measurement, **[D]** derived, **[I]** inferred.

## 1. Where our chances come from

`data/drops.txt` comes from meowdb.com's Classic World monster pages, which carry **no drop
rates** - only a community vote score (`crates/world/src/droptables.rs`, module docs). Every item chance in it is
`tools/scrape_drops.py` `chance_percent()`, a flat number per category **[L]**:

| category | our chance per row | rows |
|---|---|---|
| etc, the mob's own trophy | 40% | 130 |
| other etc | 6% | 296 |
| use - **scrolls (`204xxxx`) are "use"** | **5%** | 33 scroll rows, 227 use in all |
| setup | 3% | - |
| equip | **1%** | 278 |
| global: the three repurposed scrolls | 0.01% | 3 |

The server's drop rate multiplies every chance linearly and caps it at 100%
(`DropEntry::hits_at`), once per kill, **including the global table** **[L]**.

## 2. What other servers do

**Cosmic** (the maintained OdinMS -> HeavenMS line, v83, github.com/P0nk/Cosmic,
`src/main/resources/db/data/152-drop-data.sql`, 22 157 rows) **[L]**:

* chance is out of **1 000 000**; `MapleMap.dropItemsFromMonsterOnMap` rolls
  `Randomizer.nextInt(999999) < chance * chRate * cardRate` - the same linear multiply as ours;
* **global drops are NOT scaled by the rate** (`dropGlobalItemsFromMonsterOnMap` compares the raw
  chance) - ours are;
* the shipped `config.yaml` runs `drop_rate: 10`;
* per row, on ordinary mobs (id < 8 000 000, no quest gate):

| category | median per row | p10 - p90 |
|---|---|---|
| equip | **0.08%** | 0.07 - 0.129% |
| scroll `204xxxx` | **0.03%** | 0.03 - 0.30% |
| use (potions) | 2% | 0.8 - 5% |
| etc | 0.9% (trophies 60%) | 0.6 - 60% |
| global Chaos / White Scroll | 0.005% each | |

  A typical early mob carries **many** rows at those tiny chances: Slime 13 equips at
  0.05-0.129% and 5 scrolls at 0.03-0.075%; Horny Mushroom 23 equips.

**The local v214 reference source** (Swordie family) uses a 10 000 scale and much higher
numbers - equip median 10%, scroll 1% **[L]** - but it is modern GMS, where drops were made
deliberately generous. Not a classic-era control.

**Neither is official.** Nexon never published per-mob rates; the v83 numbers are themselves a
private-server community's calibration. They are the closest thing to a classic baseline
there is, which is what they are used for here.

## 3. Same mobs, side by side

127 of our 170 templates were matched to a Cosmic mob **by name** (our template ids are Classic
World's own numbering; GMS ids came from the reference source's `mobName` field), keeping
those whose Cosmic table has at least one equip. Expected items per kill at **1x** = the
sum of the row chances **[D]**:

| mob level | n | equips/kill: ours | v83 | ratio | scrolls/kill: ours | v83 | ratio |
|---|---|---|---|---|---|---|---|
| 1-20 | 16 | 0.061 | 0.025 | **2.5x** | 0.031 | 0.005 | **5.9x** |
| 21-40 | 58 | 0.018 | 0.016 | 1.1x | 0.010 | 0.0024 | **4.3x** |
| 41-70 | 44 | 0.008 | 0.018 | 0.5x | 0.003 | 0.003 | 1.1x |
| 71+ | 9 | 0.002 | 0.080 | 0.03x | 0 | 0.018 | - |

Examples at 1x, equips per kill: Fire Boar 0.14 against 0.014, Horny Mushroom 0.11 against
0.018, Slime 0.08 against 0.010.

**At the live server's 5x**, a level 1-20 field drops about **12x** v83's equips and **30x** its
scrolls, both measured at v83's 1x. About one Fire Boar kill in two drops an equip (14 rows at 5%: 1 - 0.95^14).

## 4. Why: a different shape, not just a different number

v83 spreads a mob's loot over **many rows at tiny chances**; ours puts it on **few rows at
large flat chances**. Per row, our equip chance is ~12x v83's median and our scroll chance is
~170x. Because our tables are short, equips per kill still come out close to v83 at levels
21-40, by coincidence of row counts. On the low-level mobs, which have long scraped lists, they
don't. Scrolls are the clearest error: they fall under "use", so they get the potion
rate of 5%.

The high-level shortfall in the last two rows of the table is the opposite problem: meowdb lists
few equips for those mobs. It is noted here, not fixed.

## 5. Options (none applied)

1. **Re-tune the policy to v83 per-kill numbers.** Give scrolls their own category at about
   **0.8%** (5% x 0.0053/0.031), and equips **0.4%** (lv 1-20: 0.061 x 0.4 = 0.024 = v83).
   That is one function in `scrape_drops.py`, plus a re-run over the committed file's score
   column (no re-scrape). Per-kill numbers at 1x then match v83 for the low and mid bands.
2. **Take v83's per-row chances where the same item drops from the same mob in Cosmic**, and use
   the policy only for the rest. That is closer for the matched rows, but it mixes two calibrations.
3. **Stop scaling the global table by the drop rate**, as Cosmic does. That covers the three
   repurposed scrolls at 0.01%, which become 0.05% at 5x today.
4. **Lower the server rate** instead. That leaves the 170x per-row scroll error in place.

## 6. Reproducing

The comparison script is not committed (it reads a third-party SQL file and a path outside the
repo). The steps are:
* parse `data/drops.txt`;
* parse Cosmic's `152-drop-data.sql` with `\((\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+)\)`, chance / 1e6;
* map names with `gm-handbook/mobnames.txt` and the reference source's `mob_drops.json` `mobName`;
* for a name with several GMS ids, take the id with the most rows.
