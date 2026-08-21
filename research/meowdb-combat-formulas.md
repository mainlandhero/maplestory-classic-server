# Damage, HP/MP gain and attack speed — the three meowdb guides

**Captured 2026-08-20**, at the owner's request: *"I also need you to take a look at the following
which is an integral part of our server ... Make sure that these make it onto the to-do
list."*

| | |
|---|---|
| damage | <https://meowdb.com/msclassic/guides/explaining-the-damage-formula> |
| HP/MP | <https://meowdb.com/msclassic/guides/hp-mp-gain-explained> |
| attack speed | <https://meowdb.com/msclassic/guides/attack-speed-and-animation-times> |

## How to treat these numbers

**Everything here is [I] — a fan site, not the client.** `CLAUDE.md`'s rule about the
reference source applies with the same force: scored against a held-out control, the modern
server tree got 1 of 8, and a fan site has no better claim by default.

**But this particular site has a track record here, and it is worth stating precisely.** It is
drawn from the same second closed online test ("COT2") this client is built from, and twice
now its data has been checked against the client's own WZ and agreed:

* all **88** citizenship quests it lists exist in this client's `Quest.wz` as `506000`-`506141`,
  and the count matches exactly;
* its per-monster drop tables are what `data/drops.txt` was built from.

So: **candidate, with a good prior.** Every number below is worth implementing against, and
every number below has to be checked against the client's own data or a measurement before it
is written down as fact anywhere else.

The damage guide labels several of its own claims unresolved. Those are marked below, because
a hedged source that names its blind spots is more useful than a confident one - and because
`CLAUDE.md` now has a whole section about what happens when a hedge gets dropped in the retelling.

---

## 1. Damage

### Physical

```text
S = SkillDamage / 100
M = (MasteryLevel / 10 + 0.1) * 0.8
B = uniform(0.8, 1.0)
Q = uniform(Primary * M, Primary)

Raw = S * TotalWATK * (B + (Q * WeaponMult + Secondary) / StatDiv + AttackPower / APDiv)

MIN = S * TotalWATK * (0.8 + (Primary * M * WeaponMult + Secondary) / StatDiv + AttackPower / APDiv)
MAX = S * TotalWATK * (1.0 + (Primary       * WeaponMult + Secondary) / StatDiv + AttackPower / APDiv)
```

Ordinary melee: `StatDiv = 100`, `APDiv = 50`.

### Primary and secondary stat by weapon

| weapon | primary | secondary |
|---|---|---|
| Sword, Axe, Blunt, Spear, Polearm | STR | DEX |
| Bow, Crossbow | DEX | STR |
| Dagger, Claw | LUK | STR + DEX |
| Wand/Staff, physical attack | INT | LUK |

### Weapon multipliers

| weapon | swing | stab | shoot | mixed |
|---|---|---|---|---|
| 1H Sword | 1.8 | 1.8 | 1.8 | 1.8 |
| 2H Sword | 2.5 | 2.5 | 2.5 | 2.5 |
| 1H Axe / Blunt | 2.4 | 1.2 | — | 1.8 |
| 2H Axe / Blunt | 3.0 | 2.0 | — | 2.5 |
| Spear | 1.5 | 3.5 | — | 2.5 |
| Polearm | 3.5 | 1.5 | — | 2.5 |
| Dagger | 1.0 | 2.0 | — | 1.5 |
| Bow / Crossbow / Claw | 1.0 | 1.0 | 2.5 | 1.0 |

An inherited ordinary melee action is **60% swing, 40% stab**.

### Magic

```text
S = SkillPower / 100
M = (SpellMastery / 10 + 0.1) * 0.8

MIN = S * MagicTotal * (1 + TotalINT * M / 100)
MAX = S * MagicTotal * (1 + TotalINT / 100)

MagicTotal = floor(TotalINT / 2) + EquipmentMATK + ScrollMATK + BuffMATK
```

### Derived stats

```text
Common = 1.2*DEX + 2*Level + 0.6*LUK

Warrior  ACC = floor(Common / 2.5 + 10)
Bowman   ACC = floor(Common / 4.8 + 20)
Thief    ACC = floor(Common / 4   + 15)
Magician ACC = floor((1.2*INT + 2*Level + 0.6*LUK) / 5.1 + 20)

EVA  = floor(LUK/3) + floor(DEX/6) + 5
WDEF = floor(STR/4)      (the part derived from stats)
MDEF = floor(INT/4)
```

### Hit chance

Outgoing:

```text
L = max(0, MobLevel - PlayerLevel)
A = TotalACC * 100 / (10*L + 255)
D = A - MobEVA
f = 0.15 + 0.20 / (1 + exp(D / 12))
R = uniform(1 - f, 1 + f)
hit when A * R >= MobEVA
```

Incoming:

```text
G = max(0, PlayerLevel - MobLevel)
A = MobACC * 100 / (5 * (G + 51))
E = PlayerEVA / (1 + PlayerEVA/80) / (1 + G/40)
FinalHitChance = CandidateChance + (1 - CandidateChance) * 0.08
```

### Criticals, defence, and the order things apply in

```text
base crit rate c = 0.05, crit multiplier d = 0.20
critical hit = normal * (1 + d)
```

1. `Raw * 100 / (EffectiveDEF + 100)`
2. element modifier, `0.00`-`1.50`
3. level-gap scale
4. critical
5. skill falloff (Iron Arrow and friends)
6. `trunc(clamp(value, 1, 99999))`

Level-gap scale:

```text
gap <= 0   ->  1
gap 1..9   ->  1 / (1 + gap^2 * 0.005)
gap >= 10  ->  1 / (1 + gap   * 0.05)
```

### Guard

```text
ShieldGuardChance = max(0.05, ShieldWDEF / (ShieldWDEF + 500))
ClawGuardChance   = 3% (Lv 1-6), 4% (Lv 7-13), 5% (Lv 14-20)
```

A successful guard fully negates a physical hit.

### Skill modifiers

| skill | modifier |
|---|---|
| Combo Attack | `1 + (5 + floor(ComboLevel/2)) / 100` |
| Panic / Coma | `1 + 5*Orbs*(Orbs-1) / 100` |
| Charge | `1 + ChargeBonus / 100` |
| Element Amplification | 135% at Lv30 (damage), 150% (MP cost) |
| Shadow Partner | 140% at Lv30 |
| Iron Arrow | `max(0, 1 - 0.2*i)` per target |
| Final Attack | proc rate * payload |
| Mortal Blow | activation rate * direct-hit damage |
| Dragon Roar | max 30 uses per 60 s; 2 s minimum interval (Lv21-30) |

### The guide's own unresolved list

Quote these when acting on them, per `CLAUDE.md`:

* the replacement rule for multiple temporary Attack Power buffs;
* borrowed-frame composite actions (Explosion, Savage Blow);
* Power Knockback close-range damage, Arrow Bomb targeting, Shadow Partner extra-hit
  attribution;
* DoT tick delivery, summons, reflection.

---

## 2. HP and MP on level up

**The site says the per-level gain is a flat number with zero variance** - *"Zero variance
across every level-up sampled, and no change between 1st and 2nd job."* That is a claim about
this game specifically; the pre-Big-Bang game most people remember rolled a range.

| class | HP / level | MP / level |
|---|---|---|
| Beginner | +16 | +12 |
| Warrior | +28 | +12 |
| Bowman | +22 | +17 |
| Thief | +22 | +17 |
| Magician | +16 | +22 |

Job advancement pays a fixed 500 points, split by class:

| class | HP | MP | when |
|---|---|---|---|
| Warrior | +350 | +150 | 1st job |
| Bowman | +250 | +250 | 2nd job |
| Thief | +250 | +250 | 1st and 2nd |
| Magician | +150 | +350 | 1st job |

Warriors and Magicians gain a further **+25% of base** from the maxed Improving Max HP / Max
MP skills.

### This contradicts what MapleCW ships, and the contradiction is the useful part

`crates/world/src/expcurve.rs` has `LevelGains { ap: 5, max_hp: 14, max_mp: 10 }` for every
character at every level. Against the table above, a Beginner should get **+16 / +12**.

Neither number is measured. Ours was a placeholder; theirs is a fan site. **The client can
settle it**: `-SetFieldProbe` already dumps the client's own EXP curve for free, and the
level-up path is the same family of tables. Until then, changing `LevelGains` to the site's
Beginner numbers would be swapping one unverified constant for another - which is worth doing
only if it is *recorded* as unverified, because the current value at least has the virtue of
being obviously provisional.

---

## 3. Attack speed and animation timing

**The page does not carry the table.** It explains the model and points at per-skill pages and
a damage simulator for the numbers, so this section is a method, not data.

* Attack timing is quantised to **30 ms steps**. The guide's justification: *"it matches
  pre-Big-Bang packet timing and a current Double Stab observation."*
* Four labels: **base animation** (the skill's unmodified action length), **animation time**
  (the modelled player-facing duration in ms), **weapon speed** (the equipped weapon's speed
  label when a skill uses its ordinary attack), and **timing range** (weapons whose several
  ordinary attacks finish at different times).
* Weapon Boosters improve speed by **two stages at every skill level**.
* Spell Booster improves F/P and I/L skills by **one** stage at levels 1-10 and **two** at 11+.
  Clerics and Priests do not get it.

**What this is for on our side**: the server currently accepts whatever attack interval the
client sends. Nothing validates it, and nothing needs to on a single-player local server - but
if attack intervals are ever checked, this is the model to check against, and the per-weapon
numbers have to come from the client's WZ rather than from this page.
