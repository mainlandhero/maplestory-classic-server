# The attack packet DOES carry the skill id, at body offset 2

**Measured 2026-08-28 from archived logs. No client run.** This retracts the claim that has
blocked every damage validator in this repo.

## 0. The retraction

`crates/world/src/magic.rs`'s module header opens with:

> **The attack packet carries no skill id.** `research/mob-combat.md` section 7 counted thirty
> header fields in `0x00DF`, five of them non-zero, and explained none; the skill is not
> among the ones that have been identified.

`research/damage-formula.md` section 9.1 says **DO NOT WIRE - nothing to connect it to** for the
same reason. Both are wrong. The field was in the very first capture; it was **zero**, because
that capture was an ordinary swing, and a zero field explains nothing about itself.

This is `CLAUDE.md`'s *"nothing new arrived is a different claim from this thing did not
arrive"*, in its other direction: nobody asked what a **skill** attack looks like. The
measurement below took one `grep` over `previous-runs/`.

## 1. The enumeration

Every `0x00DF` / `0x00E1` body in every archived world log, grouped by the `u32` at body
offset 2 and the `u8` at body offset 6:

```text
  n     opcode   off2 (u32)  off6 (u8)  attack-type string
447     0x00DF         0          0     "User Melee"
233     0x00DF         0          0     "User Melee"          (second builder)
  9     0x00DF         0          0     "User Melee"          (fixture)
  2     0x00E1      1000          3     "User Magic ..."
 14     0x00E1   2001003          7     "User Magic Skill System"
```

* `1000` is **Three Snails**, and its `maxLevel` in this client's `Skill.wz` is **3**.
* `2001003` is **Magic Claw**, and the owner had put **7** points into it - they said so on
  2026-08-27: *"I added all of the points into Magic Claw"*.

Two skills, two independent level values, each one matching a number known from a completely
different source. That is the corroboration; the id alone would only have been suggestive.

## 2. Where it sits, in the existing field map

`research/mob-combat.md` section 1.3 already enumerated the header - 39 calls plus a tail
`jmp`, no branches - and got the offsets right. Only the *meanings* of fields 2 and 3 were
open, because both are zero on a plain swing:

| # | body off | w | struct | was | **is** |
|---:|---:|---|---|---|---|
| 2 | 2 | u32 | `+0x08` | 0, unexplained | **skill id** |
| 3 | 6 | u8 | `+0x0c` | 0, unexplained | **skill level** |

Magic Claw, `world.log` 04:01:06.251, first 34 bytes of the body:

```text
00 01 6b881e00 07 00 b01862eb c1a13ab7 01 07000000 9fae3408 01 06000000 7a88791a
 |  |  |        |  |  |        |        |  |        |        |  |        |
 |  |  |        |  |  f5       f6       f7 f8       f9       f10 f11     f12 tick
 |  |  f3 = 7   f4
 |  f2 = 0x001e886b = 2001003
 f0 f1
```

`0x001e886b` is 2001003 exactly.

## 3. A second correction that falls out for free

Field 9, `+0x24`, is `0x0834ae9f` in **433 of the 434 bodies**, across 25 distinct captures and
nine days. `mob-combat.md` section 1.3 calls it *"looks like a per-attack serial / nonce
**[I]**"*. A nonce that repeats 433 times is not a nonce, so the `[I]` is refuted.

**But "it is a constant" was too strong, and that sentence is corrected here too.** There is
exactly one outlier - `previous-runs/world-20260819-222734.log` at 02:25:52.054, `0xdd01c6a9` -
and fields 8 and 34 move with it. Near-invariant and not per-attack is all that is measured.
What it *is* remains open.

```python
# counting over previous-runs/ AND research/fixtures/ without counting a capture twice
import glob, hashlib
seen, files = set(), []
for f in sorted(set(glob.glob('previous-runs/world*.log') + ['world.log']
                    + glob.glob('research/fixtures/*world*.log'))):
    h = hashlib.md5(open(f, 'rb').read()).hexdigest()
    if h not in seen:
        seen.add(h)
        files.append(f)
```

## 4. What this unblocks

| | was blocked on | now |
|---|---|---|
| `world::damage::check_hit` | the skill id | has it |
| `world::magic::check_magic_hit` | the skill id, and `skill_magic_percent` | has both - `mad` comes from `skilltable` keyed by (id, level) |
| MP cost on an attack skill | knowing which skill | `mpCon` is in `gm-handbook/skills.txt` per level |
| `attackCount` / `mobCount` / `bulletConsume` checks | same | same |

**Still true and unchanged: log only, never refuse.** The client computes and draws its own
damage; a server that disagrees has found a bug in its own model far more often than a cheat.

## 5. The fixture

**Already archived, under names given for other questions.** Two fresh copies made while
writing this file were deleted again, because a third copy of a capture is precisely what
produced the wrong counts in section 1:

| capture | fixture |
|---|---|
| 2026-08-28, the 7 Magic Claws at level 7 | `research/fixtures/magic-claw-1-damage-and-heapfix-armed-world.log` |
| 2026-08-22, the one Three Snails | `research/fixtures/cash-shop-click-sent-nothing-world.log` |

Neither name mentions the attack packet, and that is the point: **a fixture name says what its
author was looking at, not everything the file contains.** Both of these were sitting in
`research/fixtures/` with the skill id in them the whole time.

`tools/extract_attack_bodies.py` deduplicates by content hash and exits non-zero on an empty
result, so a silent zero is not one of the answers it can give.
