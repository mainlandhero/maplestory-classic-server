# The damage formula, HP/MP gain and attack speed — cross-checked against this client

**2026-08-21.** Goals **J** (damage), **K** (HP/MP per level) and **L** (attack speed).

`research/meowdb-combat-formulas.md` is the **capture of the three guides the owner named** — what
the source says. **This file is what survived contact with the client**, and it is the one to
read when the two disagree.

Labels, as everywhere in `research/`:

| | |
|---|---|
| **[L]** | read off this client — a listing, the raw instruction bytes, or a `.wz` node |
| **[D]** | derived, or measured from a capture in this repo |
| **[I]** | the meowdb guides. A fan site with a good prior on this client, not a listing |

The implementation is **`crates/world/src/damage.rs`**, 37 tests, and it is **not wired** —
see §9.

---

## 0. The headline

**The client contains the weapon-multiplier table, and it matches the guide exactly.**
`FUN_14025e000` at `0x14025e000`–`0x14025e534` is the client's damage-parameter builder. It
writes the weapon multiplier, the primary and secondary stat, the roll bounds and both
divisors into one output struct. Every number the meowdb physical formula names is in there
as a literal.

That moves a large part of goal J from **[I]** to **[L]** in one function, and it turns up
four things the guide does not have and one place the guide is wrong about this client.

| | |
|---|---|
| the whole weapon-multiplier table | **[L]**, eight weapon families, exact match |
| `B = uniform(0.8, 1.0)`, `StatDiv = 100`, `APDiv = 50` | **[L]**, the four scalars beside the multiplier |
| primary/secondary stat per weapon | **[L]** structure, **[D]** naming |
| **wand/staff physical is a STR attack, not INT** | **[L]** — the guide is wrong here |
| **Lucky Seven's multiplier is 3.0, not 2.5** | **[L]** — the guide does not have it |
| **a bow used as a club divides by 300/150, not 100/50** | **[L]** — the guide does not have it |
| **"mixed" is literally the mean of swing and stab** | **[L]** — folded at compile time, 1 ulp below `1.8` |
| **the 60/40 swing-stab split has a mechanical cause** | **[L]** — every melee weapon has 3 swing actions and 2 stabs |
| mastery is **1..=10** in this client, not a percentage | **[L]** — `Skill.wz` |
| skill damage is a **percent** | **[L]** — `Skill.wz` |
| weapon `attackSpeed` and every base animation length | **[L]** — `Character.wz` |
| **`AttackHit::flag_b` is the critical flag** | **[D]** — measured on one capture, 69 hits |

And the one that did **not** move: **HP and MP per level are still [I]**, exactly as goal K
already recorded. Nothing found here changes that, and `crates/world/src/expcurve.rs` already
carries the right table — see §7.

---

## 1. `FUN_14025e000`: the client's damage-parameter builder

### 1.1 How it was found, and why the first two scans were nearly useless

The client computes its own damage (`research/mob-combat.md` §2), so the multipliers must be
*in* the binary. A scan of all 76 MB for doubles in `{1.0, 1.2, 1.5, 1.8, 2.0, 2.4, 2.5, 3.0,
3.5}` found **one** `.rdata` cluster — and it was a **sorted compiler constant pool**, not a
table. That is the shape of a confident wrong answer: four wanted values, evenly spaced, in
`.rdata`, and meaningless.

What worked was **not filtering to clusters**. Listing every occurrence of each value
individually showed that `1.8`, `2.4` and `3.5` each occur **once or twice in the whole
image**, and all of them inside `0x14025e0c0 .. 0x14025e481` — a 1 KB span of `.text`. The
bytes are `48 B8 <imm64>` / `48 89 47 18`: `mov rax, <double>` / `mov [rdi+0x18], rax`. A
switch writing a double into one field.

> The instrument was verified before it was believed: the same scan reports `100.0` five
> times and `0.5` twenty-one times as doubles, so it can speak.
>
> **Named blind spot.** `.themida` has `SizeOfRawData` **0** — 20 MB of virtual address space
> with no bytes on disk. Nothing in this section could see a constant that lives there, so
> every negative below is "not in the 56 MB that exists on disk".

### 1.2 The output struct

`FUN_14025e000(out /*rcx*/, statObject /*rdx*/, weaponType /*r8d*/, action /*r9d*/, …,
skillId /*stack*/)`. **[L]**

```text
+0x00 i32  secondary stat (a SUM for dagger and claw)
+0x04 i32  primary stat
+0x08 f64  roll_lo    0.8    (0.2 when action == 1)      0x14327aa28
+0x10 f64  roll_hi    1.0    (0.3 when action == 1)      0x1434b95e0 / 0x1434b95c0
+0x18 f64  the weapon multiplier
+0x20 f64  stat_div   100.0  (300.0, see §1.5)           0x143277e78
+0x28 f64  stat_div, second copy — one `movups` writes both
+0x30 f64  ap_div     50.0   (150.0, see §1.5)           0x14327aad0
```

Line for line, that is meowdb's

```text
Raw = S * TotalWATK * (B + (Q * WeaponMult + Secondary) / StatDiv + AttackPower / APDiv)
```

with `B = uniform(roll_lo, roll_hi)`, `StatDiv = stat_div`, `APDiv = ap_div`. **[L]** for all
four constants; **[I]** for the shape of the expression that consumes them, which this
function does not contain.

`skillId` is identified by `cmp r15d, 0x3d0ceb` on the claw arm — `0x3d0ceb` is **4001003**,
and `Skill.wz` names 4001003 **Lucky Seven**. **[L]**

### 1.3 Weapon types are 30..=47, the same numbering `Skill.wz` uses

```asm
14025e033  lea  eax,[r8 - 0x1e]      ; weaponType - 30
14025e037  cmp  eax, 0x11            ; 18 entries
14025e03a  ja   0x14025e060          ; outside -> no multiplier at all
```

So the switch covers **30..=47** and nothing else. **[L]** That is `itemId / 10000 - 100`,
and it is the same code `Skill.wz` puts in a skill's `weapon` field — Lucky Seven carries
`47`, Double Stab carries `33`, Bow Mastery carries `45`. **[L]**

`34`, `35`, `36` are inside the range and their jump-table entries go **straight to the
exit** — no arm, no multiplier. This client ships no weapons in those classes either.

**There is no 48 (knuckle) and no 49 (gun).** `Skill.wz` likewise has no `500` job tree.
Two independent absences, one conclusion: this client has no pirates. **[L]** That also
retires the "Pirate falls back to the beginner line" placeholder question in
`crates/world/src/expcurve.rs` — nothing can reach it because the class does not exist here.

### 1.4 The table, arm by arm

Jump table at `0x14025e4ec`, index `weaponType - 30`, targets are `0x140000000 + entry`.

| wt | class | arm | swing (2) | stab (3) | shoot (4) | mixed (6) |
|---:|---|---|---|---|---|---|
| 30 | 1H Sword | `0x14025e11a` | 1.8 | 1.8 | 1.8 | 1.8 |
| 31 | 1H Axe | `0x14025e14a` | **2.4** | **1.2** | — | **1.7999999999999998** |
| 32 | 1H Blunt | `0x14025e14a` | 2.4 | 1.2 | — | 1.7999999999999998 |
| 33 | Dagger | `0x14025e1b5` | 1.0 | **2.0** | — | 1.5 |
| 37 | Wand | `0x14025e11a` | 1.8 | 1.8 | 1.8 | 1.8 |
| 38 | Staff | `0x14025e11a` | 1.8 | 1.8 | 1.8 | 1.8 |
| 39 | Bare hands | `0x14025e0f4` | 1.0 | 1.0 | 1.0 | 1.0 |
| 40 | 2H Sword | `0x14025e212` | 2.5 | 2.5 | 2.5 | 2.5 |
| 41 | 2H Axe | `0x14025e242` | **3.0** | 2.0 | — | 2.5 |
| 42 | 2H Blunt | `0x14025e242` | 3.0 | 2.0 | — | 2.5 |
| 43 | Spear | `0x14025e2ad` | 1.5 | **3.5** | — | 2.5 |
| 44 | Polearm | `0x14025e2f7` | **3.5** | 1.5 | — | 2.5 |
| 45 | Bow | `0x14025e332` | 1.0 | 1.0 | **2.5** | 1.0 |
| 46 | Crossbow | `0x14025e332` | 1.0 | 1.0 | 2.5 | 1.0 |
| 47 | Claw | `0x14025e3bf` | 1.0 | 1.0 | 2.5 | 1.0 |

**Every cell matches meowdb's table.** That is the strongest single result this fan source
has produced here, and it is worth stating precisely: the guide's *numbers* are confirmed
against the client; the *formula that consumes them* still is not.

**The action codes are 2, 3, 4 and 6.** **[L]** — the arms literally test
`sub ecx,2 / je` … `cmp ecx,3 / je`. `5` is tested by nothing. The **names** swing / stab /
shoot / mixed are **[D]**, from matching the axe arm (`2 -> 2.4`, `3 -> 1.2`) and the bow arm
(`4 -> 2.5`) against the guide's columns — which agree for all eight families, so the naming
is over-determined.

**Action `1`** forces the multiplier to `1.0` at the exit (`cmp ebp,1 / jne / mov [rdi+0x18],
r14`) and swaps all four scalars at the head. **It is deliberately not called "magic" here.**
The client says nothing more about it than that, and naming it would be a guess.

#### "Mixed" is the arithmetic mean, and the evidence is one bit

The 1H axe/blunt mixed constant at `0x14025e17c` is `0x3FFCCCCCCCCCCCCC` — **one ulp below**
the literal `1.8` at `0x14025e137` (`0x3FFCCCCCCCCCCCCD`). `2.4 + 1.2` is an exact tie between
two doubles; round-half-to-even takes the lower; halving it gives exactly the constant in the
binary. So the compiler folded `(2.4 + 1.2) / 2` and the "mixed" column is **the mean of swing
and stab**, not an authored number. **[L]**

That also settles a thing the guide leaves ambiguous. meowdb lists a "mixed" column *and*
says an inherited ordinary melee action is 60% swing / 40% stab. Those are two different
things: with a 60/40 blend a 1H axe would be `0.6*2.4 + 0.4*1.2 = 1.92`, not 1.8. The stored
"mixed" is the 50/50 mean.

#### The 60/40 split has a mechanical cause

**[L]**, from `Character.wz/Weapon`: every melee weapon image carries exactly **three ordinary
swing actions and two ordinary stabs**.

| class | swings | stabs |
|---|---|---|
| 1H sword / axe / blunt / dagger / claw | `swingO1`, `swingO2`, `swingO3` | `stabO1`, `stabO2` |
| 2H sword / axe / blunt | `swingT1`, `swingT2`, `swingT3` | `stabO1`, `stabO2` |
| spear / polearm | `swingP1`, `swingP2`, `swingT2` | `stabT1`, `stabT2` |

Three of five is **60%**. Bows carry only `shoot1`, crossbows only `shoot2`.

#### The inherited action, and when it actually fires

The *other* jump table, at `0x14025e4a4`, picks a default action — but the head gates it:

```asm
14025e02a  test r15d,r15d / je 0x14025e062   ; skillId == 0  -> SKIP the default table
14025e02f  test ebp,ebp   / jne 0x14025e062  ; action != 0   -> SKIP
```

So it runs **only for a skill that names no action of its own**. Every entry is the weapon's
*higher* multiplier — axe swing 2.4 over stab 1.2, dagger stab 2.0 over swing 1.0, spear stab
3.5, polearm swing 3.5, bow/crossbow/claw shoot. **[L]** That symmetry is a useful check on
the transcription: a mis-copied arm would break it.

A plain no-skill attack therefore arrives with an explicit action, and that is where the 60/40
lives.

### 1.5 Primary and secondary stat, and the row where the guide is wrong

Each arm calls a stat getter (`FUN_1401ba9d0(ptr, cap)`) and writes the results to `+0x04`
(primary) and `+0x00` (secondary). It reads the stat object at `+0x24`, `+0x30` and `+0x48` —
an even `0xc` stride whose fourth slot `+0x3c` **this function never touches**. **[L]**

| class | primary | secondary | offsets read |
|---|---|---|---|
| 1H/2H sword, axe, blunt, spear, polearm, **wand, staff**, bare hands | STR | DEX | `+0x24`, `+0x30` |
| bow, crossbow | DEX | STR | `+0x30` then `+0x24` |
| dagger, claw | LUK | STR **+** DEX | `+0x48`, then `+0x24` and `+0x30` summed |

**[D] for the naming**, and it is over-determined rather than assumed: a bow's primary is
`+0x30` while a sword's primary is `+0x24`, and a dagger's primary is `+0x48` with
`+0x24 + +0x30` as its secondary. Under STR/DEX/INT/LUK on that stride, that is sword→STR,
bow→DEX, dagger→LUK with STR+DEX secondary — three families agreeing with the guide
independently.

> ### The guide is wrong about wands
>
> meowdb gives wand and staff a physical attack of `(INT * 1.8 + LUK) / 100`. **This client
> sends weapon types 37 and 38 to jump-table entry `0x14025e11a` — the 1H sword's arm.** It
> reads `+0x24` into primary and `+0x30` into secondary. **INT is never read by this function
> at all.**
>
> So a wand's physical swing is a **STR** attack in this client, with DEX secondary and a 1.8
> multiplier. Magic damage is a different path entirely and is not in this function.

#### The bow-as-a-club case, which the guide does not have

meowdb says "for melee/fired attacks: StatDiv = 100, APDiv = 50". That is right for a bow's
**shoot**, which is the case that matters. It is **not** what the client does when a ranged
weapon plays a melee action:

| class + action | `stat_div` | `ap_div` | where |
|---|---:|---:|---|
| any melee weapon, any action | 100 | 50 | head, `0x14025e090` |
| bow/crossbow/claw, **shoot** | 100 | 50 | arm jumps straight to the exit |
| bow/crossbow/claw, **swing** or **mixed** | **300** | **150** | `0x14025e36d`, `0x14025e40b` |
| bow/crossbow/claw, **stab** | **300** | 50 | `0x14025e3a8`, `0x14025e48f` — writes `+0x20`/`+0x28` only |
| **any**, action `1` | **300** | **150** | head, `0x14025e06e` |

The stab asymmetry — `stat_div` tripled, `ap_div` left alone — is what the instructions do,
not a transcription slip. **[L]**

### 1.6 Lucky Seven

```asm
14025e42f  cmp  r15d, 0x3d0ceb        ; skill 4001003
14025e436  jne  0x14025e455
14025e438  ...  re-read the LUK stat into +0x04
14025e447  movabs rax, 0x4008000000000000   ; 3.0
14025e451  mov  [rdi+0x18], rax
```

**[L]** The one skill in the whole function with its own multiplier: **3.0**, not the claw's
ordinary 2.5. The guide does not have this.

---

## 2. Skill damage and mastery, out of `Skill.wz` — and both units were wrong by default

`client-patched/Data/Skill/Skill_000.wz`, 25 job images plus `Attacktype.img` and
`ItemSkill.img`. Enumerating every scalar key in every skill's `level/1` node — 176 skills —
gives, among 50 keys: `damage` on 45, `mastery` on 25, `mpCon` on 117, `attackCount` on 48,
`prop` on 52, `dot`/`dotTime`/`dotInterval` on 8–9.

**`damage` is a percentage. [L]** Power Strike (`1001001`) is `160, 165, 170, …, 260` across
its twenty levels; Slash Blast is `70 … 130`. That is meowdb's `S = SkillDamage / 100`, and
the unit is confirmed rather than assumed.

**`mastery` is 1..=10 in this client, and this is the important one. [L]** Every mastery skill
— Sword, Axe, Blunt, Spear, Polearm, Bow, Crossbow, Claw, Dagger and fifteen spell masteries —
runs `1,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10`.

> **This is exactly the trap `CLAUDE.md` names.** In the MapleStory versions most references
> describe, `mastery` is a **percentage** and tops out at 60. Had that unit been assumed here,
> meowdb's `M = (MasteryLevel / 10 + 0.1) * 0.8` would have produced `4.88` for a maxed
> mastery instead of `0.88` — a **5.5x** error in the minimum damage, and a "the damage
> formula is broken" bug on screen. The guide's divisor of 10 is this client's own unit, and
> that agreement is itself a point in the guide's favour.

`mastery = 0` — a character with no mastery skill — gives `M = 0.08`. **That is zero mastery,
not "unset".** `crates/world/src/damage.rs` says so at the function.

`Attacktype.img` is **not** a weapon table: it is the status/DoT art table keyed on effect id
(`1000` is `독`, poison), carrying `time` and `dmg` per level. Checked so nobody re-checks it.

---

## 3. Measured against 361 real client-computed hits

Every `0x00DF` in `previous-runs/world*.log` and `research/fixtures/*world*.log` was re-parsed
with the shipping parser's own offsets. **342 bodies are 229 bytes, 19 are 221, and 54 are the
empty 127** — 361 with a target, and every one of them carries exactly **one** target with
exactly **one** damage.

> The empty 127-byte bodies are the old "the client will not target our mobs" era, before the
> mob-size fix. They are not a parse failure.
>
> Note that `research/fixtures/skill-window-close-faults-world.log` and
> `previous-runs/world-20260820-181822.log` are **the same capture** — a fixture copy — and
> produce identical numbers. Counting them twice would inflate any pooled figure, which is one
> more reason §3.2's conclusion is drawn from a single named file.

Two things fall straight out, and both are free.

### 3.1 The mob's object id is the FIRST `u32` — a question `net::combat` marks `[I]`

`crates/net/src/combat.rs` says of `AttackTarget`: *"Which field is the mob id is not
established … `[t+0x10]` is first, which is the order the reference server uses — **[I]**"*,
and *"One client run settles it: object ids on map 40 are 2000-2039."*

**The runs already happened.** Across all captures the first `u32` takes the values
**2000, 2001, 2002, 2003, 2004, 2006, 2007, 2009, 2013, 2016, 2022, 2032, 2033, 2034** —
every one of them an id this server assigned on map 40. The second `u32` is `2` or `0`.

So `AttackTarget::object_id` is right, and the label can go from **[I]** to **[D]**. No client
run was spent on it.

### 3.2 `flag_b` is the CRITICAL flag

`AttackHit` carries two unexplained `u8` flags, with a note that *"in this game family one of
them is 'critical' — that is **[I]** and nothing here reads it."*

**One capture**, `previous-runs/world-20260820-181822.log`, one character, one session, 69
hits — no cross-session pooling:

| | |
|---|---|
| `flag_b == 0` | **64** hits, damages **15, 16, 17, 18, 19, 20** |
| `flag_b == 1` | **5** hits, damages **20, 21, 22, 24** |
| rate | 5 / 69 = **7.2%** |

* **Every flagged hit is at or above the maximum of the 64 unflagged ones, and one is above
  it.** A hit of 24 cannot be a draw from a distribution whose observed maximum is 20, so the
  flag is not noise.
* `{trunc(b * 1.2) : b in 15..=20}` is `{18, 19, 20, 21, 22, 24}`, and all four observed crit
  values are in it.
* 7.2% of 69 is consistent with a 5% base rate (95% interval roughly 2%–16%).

**So `flag_b` is the critical flag. [D]** `flag_a` was `0` on all 361 hits and remains
unexplained.

**What is *not* settled is the multiplier.** In the pooled 361-hit corpus, 7 of 15 crits sit at
the very top of the predicted set, where only 8.7% of non-crits sit — a skew a uniform re-roll
would not produce. Differing mob defence per template is the leading explanation and that data
does not control for it. So: `×1.2` is **consistent**, not measured.

### 3.3 The physical window, checked against one capture

`previous-runs/world-20260821-001440.log` is character **208 "Programmer"**: level 7, job 0,
**STR 7, DEX 7**, INT 5, LUK 6, **30 AP unspent** — six level-ups at five AP each, so the stats
are still the creation roll — weapon slot 11 = **`1312000`**, a 1H axe with `incWAT` **17**, no
mastery skill, no buff. Fourteen hits, every target a template 1 or 2 mob whose `PDDamage` is
**0**.

```text
swing (mult 2.4)   MIN 15.018   MAX 21.046
stab  (mult 1.2)   MIN 14.904   MAX 19.618
union, truncated                14 .. 21
observed (n = 14)               16,16,16,16,16,16,17,17,17,18,18,18,19,19
```

**Every hit is inside the window.** It is pinned as a test in `damage.rs`.

> **This is a containment check, not a fit, and calling it a confirmation would be the exact
> mistake this repo keeps making.** Fourteen samples spanning 16..19 inside a predicted 14..21
> would also sit inside a formula that was 20% wrong in either direction. It is recorded
> because it was free and because a *failure* would have been decisive. §10 says what a real
> test looks like.
>
> The other 347 hits are **not** pooled into this. TestCharD (id 204) produced most of them
> and has since been **deleted from the database**, so its STR, DEX and weapon at the time are
> unrecoverable. Using them would be exactly the "two different sessions, one conclusion"
> failure `CLAUDE.md` records.

---

## 4. The rest of the outgoing model — still [I], with the constants located

None of the following is in `FUN_14025e000`, and none of it was traced further. All **[I]**,
from the guide.

```text
after defence  raw * 100 / (EffectiveDEF + 100)
element        0.00 / 0.25 / 0.50 / 0.75 / 1.00 / 1.25 / 1.50
level gap      gap <= 0 -> 1 ; 1..9 -> 1/(1 + gap^2*0.005) ; >= 10 -> 1/(1 + gap*0.05)
critical       normal * (1 + 0.20), base rate 0.05
final          trunc(clamp(value, 1, 99999))
```

One observation that raises the prior and settles nothing. A single sorted `.rdata` constant
pool, `0x14327a9d0`–`0x14327ab18`, contains as doubles:

```text
0.005  0.02  0.08  0.1  0.15  0.2  0.25  0.4  0.75  0.8  0.85  1.2  1.25  1.5
5  8  12  15  18  20  25  30  40  50  80  125  ...  99999  50000000
```

Every constant the guide's damage, hit-chance, guard and incoming-damage models name is in
that list, and most occur **once in the whole 76 MB image** — including **`99999.0` at
`0x14327ab10`** and **`50000000.0` at `0x14327ab18`**, the two damage caps, each exactly once.

> **This is adjacency in a compiler-merged, value-sorted pool, and it proves nothing about any
> particular formula.** The pool is every double the image uses. What it *does* say is that
> these specific values are used by this client somewhere, which a wholesale fabrication would
> not produce. Treat it as a raised prior, not evidence. The two caps are the exception — a
> cap is a distinctive number and `99999` occurring once is a real find.

**`51.0` is absent**, which is fine: the guide's `5 * (G + 51)` would be computed in integers,
and `255.0` — the outgoing `10*L + 255` — *is* present, at `0x1434b9600`.

---

## 5. Mob → player: the one place the server is the authority

The client→server "I was hit" packet has never been found (`research/mob-combat.md` §14), so
the server applies touch damage itself and the HP bar moves because we send `0x007C`. **This
is the only damage number on this server that is ours.**

**What is shipping today**, `net::combat::touch_damage`:

```text
damage = PADamage * (1 + 5% per level of gap), floored at 1
```

**What the guide gives**, **[I]**:

```text
U      = uniform(0, 1)
Roll   = 1.1 + 0.4 * U                                   <- a MULTIPLIER
Raw    = mobAttack * Roll
Taken  = Raw * (1 - DEF / (DEF + 5 * (playerLevel + 40) + 1.2 * Raw))
result = trunc(clamp(Taken, 1, 50_000_000))
```

`mobAttack` is `PADamage` and `DEF` is the player's WDEF — `floor(STR/4)` plus equipment.
`PADamage` is **[L]** out of `gm-handbook/mobtemplates.txt`.

> ### The level term goes the way you do not expect
>
> `playerLevel` sits in the **denominator of the defence ratio**, so raising it *shrinks*
> `DEF / denominator` and the player takes **more** from the same mob at the same DEF.
> Levelling still helps — through the DEF and HP it buys — but this term makes a point of
> defence worth less the higher you are.
>
> That reads like a transcription error and it is not one: it is what the guide's line says,
> and `5`, `40` and `1.2` are all doubles in this client. It is called out because "fixing" it
> to match intuition would be the exact move this repo has paid for before. `damage.rs` has an
> assertion that fails if someone flips it. **What would settle it: one session at two levels,
> same gear, same mob.**

`damage::incoming_damage` implements it and takes `roll` as a parameter, so it stays pure and
a test can pin both ends. **It is not connected to anything** — see §9.

---

## 6. Attack speed and animation timing (goal L)

### 6.1 The guide does not carry the table — but the damage guide carries the model

The attack-speed page is a method, not data, exactly as goal L records. **The damage page
carries the arithmetic**, and this is new since the 2026-08-20 capture:

```text
timingTier  = clamp(resolvedTier, 0, 10)
ScaledFrame = truncTowardZero(FrameDelay * (10 + timingTier) / 16)     <- per frame
nativeMs    = sum(ScaledFrame)
AnimationTime = 30 * ceil(nativeMs / 30)                               <- milliseconds
```

**[I]**, and the guide hedges its own outer cadence: *"Ordinary per-frame timing is confirmed.
The outer 30 ms player-facing cadence is best-supported, while borrowed-frame composite actions
remain provisional."* Quoted here because acting on a hedge without repeating it is how the
touch-damage retraction happened.

### 6.2 Both inputs are [L] in this client

**Frame delays**, milliseconds, from `Character.wz/Character_000.wz/`**`00002000.img`** — the
body image. A *weapon* image carries only sprite placement per frame and no `delay` at all,
which is why looking there finds nothing.

| action | delays (ms) | total |
|---|---|---:|
| `swingO1` / `O2` / `O3`, `swingT1..T3`, `swingP1` / `P2`, `shoot1` | 300, 150, 350 | 800 |
| `swingOF` | 200, 100, 100, 300 | 700 |
| `swingTF` | 200, 150, 150, 200 | 700 |
| `swingPF`, `stabTF` | 100, 200, 200, 200 | 700 |
| `stabO1` / `O2` | 350, 450 | 800 |
| `stabOF` | 250, 150, 300 | 700 |
| `stabT1` / `T2` | 300, 100, 350 | 750 |
| `shoot2` | 160, 160, 250, 100, 150 | 820 |
| `shootF` | 300, 150, 250 | 700 |
| `proneStab` | 300, 400 | 700 |

> **Ten more actions carry NEGATIVE delays and are deliberately excluded**: `shoot6`
> (`-480, -160, 260`), `shootDb1`, `shotC1`, `magic1` (`-900, 200, 200`), `magic2`, `magic3`,
> `savage`, `assaulter`, `avenger`, `burster1`, `burster2`. What a negative `delay` means to
> this client's renderer is **not established**, and summing `|delay|` versus dropping the
> frame gives answers up to 1.8x apart. That is a unit question with a wrong answer waiting in
> it, so they are named and left out.

**Weapon speed**, from `Character.wz/Weapon` — `info/attackSpeed` on all 203 real weapons:

| class | attackSpeed values (count) |
|---|---|
| 130 1H Sword | 4 ×15, 5 ×4 |
| 131 1H Axe | 4 ×8, 5 ×3 |
| 132 1H Blunt | 5 ×21, 4 ×5, 2 ×1 |
| 133 Dagger | 4 ×13, 3 ×9 |
| 137 Wand | 6 ×11 |
| 138 Staff | 7 ×8 |
| 139 Bare hands | 6 ×1 |
| 140 2H Sword | 5 ×7, 6 ×4 |
| 141 2H Axe | 5 ×7, 6 ×3 |
| 142 2H Blunt | 7 ×8, 6 ×2 |
| 143 Spear | 7 ×5, 6 ×4 |
| 144 Polearm | **8 ×9**, 7 ×1 |
| 145 Bow | 6 ×11, 5 ×2 |
| 146 Crossbow | 7 ×8, 6 ×2 |
| 147 Claw | **4 ×28**, 5 ×2, 3 ×1 |

Range **2..=8**, and the ordering is exactly what the game is known for: claws and daggers
fastest, polearms slowest. **[L]**

### 6.3 The tier is the weapon's own `attackSpeed` — [D], on one arithmetic anchor

`(10 + tier) / 16` equals **1 exactly at tier 6**, and it is the only value that makes the
scaling a no-op. `6` is a real `attackSpeed` in this client — wands, bows, several two-handers
and the bare-hand weapon all carry it. So reading meowdb's `resolvedTier` as the weapon's
`attackSpeed` is **[D]**, resting on that one anchor plus the direction (lower is faster) which
the per-class table above independently confirms.

Two worked numbers, both landing suspiciously round:

```text
polearm swing, attackSpeed 8:  trunc(300*18/16)=337 + trunc(150*18/16)=168 + trunc(350*18/16)=393
                             = 898 ms  ->  30 * ceil(898/30) = 900 ms
1H sword swing, attackSpeed 4: 262 + 131 + 306 = 699 ms  ->  720 ms
```

A round 900 is weak evidence and is offered as such.

### 6.4 `gm-handbook/equips.txt` is missing two columns, and that is a real gap

`tools/dump_equips.py`'s docstring says the column set is *"enumerated, not assumed"*. It was
enumerated **once, by hand**, and then hard-coded as `COLUMNS`. Re-enumerating every scalar
`info` key over all **1 760** equip images (0 unreadable) finds **`attackSpeed` on 203 weapons
and `attack` on 203** — neither carried into the generated file.

* **`attackSpeed`** is the input to everything in §6. Without it the server has no way to know
  a weapon's speed.
* **`attack`** is the *animation family*, `1..=8`: `1`={130,131,132,133}, `2`={143,144},
  `3`={145}, `4`={146}, `5`={140,141,142}, `6`={137,138}, `7`={147}, `8`={139}. Note it does
  **not** separate a sword from an axe from a dagger, so it is **not** the multiplier key —
  the item-id class is. Worth carrying anyway, and worth writing down so nobody mistakes it.

Also missing and worth having: `reqLevel`, `reqSTR/DEX/INT/LUK`, `reqJob` (present on nearly
every equip), and `knockback` (23 weapons). The generator change is one edit to `COLUMNS` plus
the header line.

---

## 7. HP and MP (goals D and K) — nothing moved, and that is the finding

**`crates/world/src/expcurve.rs` already carries the guide's table**, and it is correct against
the freshly-refetched page:

| class | HP/level | MP/level | in `expcurve.rs`? |
|---|---:|---:|---|
| Beginner | +16 | +12 | yes |
| Warrior | +28 | +12 | yes |
| Bowman | +22 | +17 | yes |
| Thief | +22 | +17 | yes |
| Magician | +16 | +22 | yes |

**So there is no diff to hand the coordinator.** `crates/world/src/damage.rs` deliberately does
**not** duplicate these — a second copy is a second place to edit, which is the opposite of what
goal E asked for — and carries a test that fails if someone adds one.

Three things the re-fetch added:

* **Base pools: 50 HP and 5 MP at creation.** The guide says so; `net::opcode::Character`'s
  `Default` has said `hp: 50, max_hp: 50, mp: 5, max_mp: 5` since before that guide was read
  here. Two independent [I] sources on the same number. Still [I]; recorded as
  `damage::BASE_MAX_HP` / `BASE_MAX_MP` with a test asserting they agree with what the server
  actually creates.
* **A Beginner accuracy line the earlier capture missed**: `floor(Common / 2.5 + 5)`, where
  `Common = 1.2*DEX + 2*Level + 0.6*LUK`. Every character on this server is a beginner, so this
  is the only line that could matter — and nothing uses it, because the client owns hit chance.
* **Methodology, in the author's words**: the values were measured *"off the status bar frame
  by frame in public test footage"*. That is a behavioural measurement of the live service, not
  a listing, and it is why goal K's `[I] with a good prior` is the right label.

**Still not implemented, on purpose** — both are in this file so they are findable, and neither
is a table in the code, because `CLAUDE.md`'s "built is not wired" says an unwired table is
worse than a documented absence:

| | HP | MP | when |
|---|---:|---:|---|
| Warrior | +350 | +150 | 1st job |
| Bowman | +250 | +250 | 2nd job |
| Thief | +250 | +250 | 1st and 2nd |
| Magician | +150 | +350 | 1st job |

Plus **+25% of base** from maxed Improving Max HP / Max MP for warriors and magicians. Job
advancement is goal E and does not exist, so neither has anything to hang off.

**The client still cannot settle any of this**, exactly as goal K established, and nothing in
this pass changes that: `Skill.wz` has no per-level HP/MP node, `Character.wz` has none, and the
EXP curve at `0x143AC2400` has no sibling table.

---

## 8. `crates/world/src/damage.rs`

Pure functions, one named table per rule, 37 tests, no I/O and no dependency on the session.

| item | what |
|---|---|
| `WeaponClass` | 30..=47, `from_item_id`, the three codes with no arm |
| `AttackAction` | 1 / 2 / 3 / 4 / 6, the raw integers |
| `weapon_multiplier`, `weapon_multiplier_for` | §1.4, including Lucky Seven. `Option<f64>`, and `None` means *the client computes zero here* |
| `Scalars`, `scalars` | the four constants, including the bow-as-a-club case |
| `Stats`, `primary_and_secondary` | §1.5, wand included |
| `mastery_factor`, `MASTERY_MAX` | 1..=10, and `0` means zero |
| `Attacker`, `physical_window` | the `[min, max]` window, raw damage points |
| `after_defence`, `level_gap_scale`, `CRIT_*`, `finish`, `DAMAGE_CAP` | §4 |
| `max_plausible_hit`, `check_hit`, `HitVerdict` | validation, §9 |
| `incoming_damage`, `incoming_window`, `wdef_from_strength`, `evasion` | §5 |
| `ORDINARY_ACTIONS`, `timing_tier`, `animation_time_ms`, `min_attack_interval_ms` | §6 |
| `BASE_MAX_HP` / `BASE_MAX_MP`, `HP_MP_PER_LEVEL_LIVES_IN` | §7 |

The module needs `pub mod damage;` in `crates/world/src/lib.rs`. **That line has not been
added** — the coordinator owns `lib.rs`.

The tests were run in a **private `CARGO_TARGET_DIR`**, not the shared `target/`, so the result
did not race the four other agents' builds: **37 passed, 0 failed, no warnings.** The whole
workspace still needs a run once everyone has reported.

---

## 9. WIRE IT LIKE THIS

**Two of the four pieces should not be wired at all today, and saying so is the answer.**

### 9.1 Outgoing damage — SUPERSEDED 2026-08-28. The skill field was found; wire it as logging.

> **Retraction.** This section said DO NOT WIRE because two of the four validator inputs were
> missing. **One of them is now measured**: the skill id is the `u32` at body offset 2 and its
> level is the `u8` at offset 6 - `research/attack-skill-id.md`, corroborated by two skills
> whose levels were known independently. The *action* field is still unfound, so the paragraph
> below about `max_plausible_hit` still describes the fallback; it is no longer the only
> option. With the skill id in hand `check_hit` gets a real per-skill ceiling, and
> `magic::check_magic_hit` - which needed both the id *and* `skill_magic_percent` - becomes
> callable for the first time, since `mad` is keyed by (skill id, level) in `skilltable`.
>
> The evidence for the old claim was **an absence in captures that could not have contained
> the thing**: every archived body was an ordinary swing, where the field is legitimately `0`.
> That is not a negative result, and it should not have been written as one.

The client computes and sends the number. The only server-side use is validation, and
validation needs three inputs:

| input | have it? |
|---|---|
| the weapon | **yes** — `store` knows equip slot 11, and `WeaponClass::from_item_id` |
| the stats | **yes** — the character row, plus `equips.txt` for `incWAT` |
| **which action** was played | **no** — one of the 30 unexplained `0x00DF` header fields |
| **which skill** was used | **no** — same |

Without the last two, `check_hit` can only use `max_plausible_hit`, which maximises over every
action *and* applies the crit multiplier. For character 208's axe that ceiling is **25** against
an observed 19 — it would catch a client claiming 500, and nothing subtler. **That is a
cheat-detection tripwire, not a damage authority, and there is no cheating client here.**

If it is wired anyway, wire it as **logging only**:

```rust
// crates/world/src/session/, wherever parse_attack's targets are already walked
let class = weapon_item_id.and_then(damage::WeaponClass::from_item_id);
if let damage::HitVerdict::TooHigh { ceiling } =
       damage::check_hit(hit.damage, &attacker, class, 0) {
    eprintln!("implausible hit {} > {ceiling}", hit.damage);   // and then apply it anyway
}
```

**Never refuse the packet.** A validator that returns an error in place of a reply is the
always-answer rule broken, and the mob would stop dying.

The genuinely useful first step was **finding the action and skill fields in the `0x00DF`
header**, which turns the ceiling from 25 into a real window of 15–21 and makes the check
discriminating. **The skill half is done** (`research/attack-skill-id.md`); the action half is
not. `research/mob-combat.md` §1.3 now names fields 2 and 3 and §7 lists what is still open.

### 9.2 Mob → player damage — READY, but it is a behaviour change on working code

`damage::incoming_damage` is a drop-in for `net::combat::touch_damage`, which is called from
`crates/world/src/session/` (owned by the coordinator). The diff:

```rust
// today
let damage = combat::touch_damage(template.pa_damage, template.level, character.level);

// with this module
let wdef = damage::wdef_from_strength(character.strength);   // + equipment incPDD
let roll = damage::INCOMING_ROLL_LO + damage::INCOMING_ROLL_SPAN * rng();
let damage = damage::incoming_damage(template.pa_damage, character.level, wdef, roll);
```

**What changes on screen**: a level-1 snail (`PADamage` 3) against a level-1 character with
STR 12 goes from a flat **3** to a rolled **3..4**. Small. At higher levels the difference is
larger and the direction is *more* damage, because of the level term in §5.

**This is a live, confirmed-working feature** — *"I'm taking damage, and the mob is also taking
damage"* — and it needs an `rng`, which `touch_damage` has never had. It is the coordinator's
call, and worth doing only alongside something that makes the change observable.

**Do not do both this and §9.1 in the same run.** One variant at a time.

### 9.3 Attack-speed validation — DO NOT WIRE.

`min_attack_interval_ms` is a floor for a rate check that does not exist and does not need to
on a single-player local server. It is here because the owner asked for goal L and because the
inputs are now [L]. Wiring it would add a way to reject a legitimate swing and gain nothing.

**The one thing worth doing from §6 costs no client run**: add `attackSpeed` and `attack` to
`tools/dump_equips.py`'s `COLUMNS` (§6.4). Until then no caller can supply a real speed.

### 9.4 Three cheap corrections to files this agent does not own

* **`crates/net/src/combat.rs`, `AttackTarget::object_id`** — the `[I]` note can become `[D]`.
  §3.1: the ids in every capture are 2000–2034, which are ours.
* **`crates/net/src/combat.rs`, `AttackHit::flag_b`** — *"one of them is 'critical' — that is
  [I]"* becomes **[D]**, measured. §3.2.
* **`crates/world/src/expcurve.rs`** — `MAX_LEVEL` is **100**, but the test right below it now
  asserts the curve funds **119** levels from the client's own dumped table. `award()` stops at
  100, so levels 101–119 are unreachable despite the client paying for them. Unrelated to this
  workstream and not touched; flagged because it was read in passing.

---

## 10. The single discriminating client test

**There is one, it is cheap, and it produces a real number rather than a containment check.**

Everything else here is either invisible (the client draws its own damage) or already answered
by captures on disk. What is *not* answered is whether the physical formula is right to better
than ±20%, and §3.3 explains why: the only character with known stats produced 14 hits into a
7-point-wide window.

**What the owner does**, from an elevated window:

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

Enter the world as **Programmer** (id 208 — level 7, STR 7, DEX 7, the `1312000` axe), go to
**map 40**, and **swing at snails until about forty hits have landed** — roughly a minute of
attacking. Nothing to watch on screen; the evidence is `world.log`.

**Then, from the repo** (`cd C:\MapleCW` first — an elevated window opens in
`system32`), count the damages in the `0x00DF` bodies.

| outcome | what it means |
|---|---|
| the **minimum** lands on **14 or 15** and the **maximum** on **20 or 21** | the window is right at both ends. The formula is confirmed to the resolution the data allows, and `[I]` on the physical shape can become `[D]` |
| the range is **narrower** at both ends, e.g. 16..19 again with 40 hits | the roll is narrower than `uniform(0.8, 1.0) × uniform(P·M, P)` — most likely the **mastery** term: `M = 0.08` for no mastery is the widest single assumption in the model |
| anything lands **outside 14..21** | the formula is wrong, and by how much and in which direction says where. Above 21 with no crit flag points at the multiplier; below 14 points at `roll_lo` or at defence being applied where we think it is not |
| the **crit flag** appears on roughly 2 of 40 | 5% holds at a second sample size |
| every crit is exactly `trunc(normal_max × 1.2)` and never lower | §3.2's open question resolves the *other* way — crits use the top of the roll, not a fresh one |

**Why this one and not another.** It costs a launch the owner was going to spend anyway on the
heap-corruption repro, it needs no server change, no new packet and no probe, and it is the
only measurement on this list whose outcome could **falsify** something. The character's stats
are known and unspent, the mobs' defence is zero, and the predicted window is already written
down as an assertion — so the run either confirms it or names the term that is wrong.

**Do not combine it with a `touch_damage` change** (§9.2): that would alter the incoming
numbers in the same session and put two variables in one run.

---

## 11. What is still open

* **The action and skill fields in the `0x00DF` header.** The single highest-value thing left
  for goal J: without them, validation has a ceiling instead of a window.
* **The crit multiplier**, §3.2 — consistent with 1.2, skewed in a way 1.2 alone does not
  explain.
* **The level term's direction in incoming damage**, §5 — transcribed, counterintuitive,
  untested.
* **Negative frame delays**, §6.2 — ten actions, no established meaning, a 1.8x unit hazard.
* **Magic damage.** `FUN_14025e000` is the *physical* parameter builder; nothing here touched
  the magic path, and `MagicTotal = floor(TotalINT/2) + MATK` is still purely **[I]**. No
  character on this server can cast anything.
* **Everything meowdb lists as unresolved** — Attack Power buff replacement, borrowed-frame
  composite actions, Power Knockback close range, Arrow Bomb targeting, Shadow Partner
  attribution, DoT delivery, summons, reflection. Quoted in
  `research/meowdb-combat-formulas.md`; none of it is reachable on this server.
* **`.themida`**, 20 MB with no bytes on disk, is the standing blind spot behind every negative
  in §1 and §4.
