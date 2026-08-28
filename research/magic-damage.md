# Magic damage, and the two first-job Magician buffs

**2026-08-27.** The owner: *"I would like to know that they are both functional and damaging enemies
correctly."*

`research/damage-formula.md` §11 lists **"Magic damage"** as open: *"`FUN_14025e000` is the
physical parameter builder; nothing here touched the magic path, and `MagicTotal =
floor(TotalINT/2) + MATK` is still purely **[I]**."* It is not [I] any more. The magic path is
a **separate function**, `FUN_14025ffd0`, and every term in it is now read off the listing.

Labels, as everywhere in `research/`:

| | |
|---|---|
| **[L]** | read off this client — a listing, the raw bytes, a `.wz` node |
| **[D]** | derived from two or more [L] facts |
| **[I]** | inferred; the meowdb guides, or the v214 reference tree |

**No Ghidra** (three other agents running, single-process lock), **no `cargo`**, **no `git`**,
**no client run.** Nothing below is wired; §9 says what to wire and §10 says what is still a
guess.

---

## 0. The headline

| question | answer |
|---|---|
| Where is magic damage computed? | **`FUN_14025ffd0`**, `0x14025ffd0..0x140260a0a`, 2618 bytes. Sibling of the physical `FUN_14025f380`; the two are called from the **same four call sites** with **identical arguments** except the request struct. **[L]** |
| Does it use `FUN_14025e000`? | **No.** The physical parameter builder has five callers and `FUN_14025ffd0` is not one of them. No weapon multiplier, no primary/secondary pair, no `StatDiv`/`APDiv`, no `AttackPower` term. **[L]** |
| The formula | `raw = (skillMagic% / 100) * MagicTotal * (1 + Q/100)`, `Q = uniform(INT*M, INT)` **[L]** |
| Mastery | **the same function as physical** — `FUN_14025e840` returns `(mastery/10 + 0.1) * 0.8`, and the divisor `10.0`, addend `0.1` and factor `0.8` are three separate `.rdata` doubles. **[L]** |
| `MagicTotal` | the attacker-totals field at `+0x08`, which `FUN_14087c130` **seeds with `floor(INT/2)`** and then adds equipment and CTS 85 to. The guide's `floor(TotalINT/2) + MATK` is **exactly right**, and it is now **[L]**, not [I] |
| Post-processing | defence → element → level gap → damage-up buff → critical → `trunc(clamp(v, 1, 99999))`. **Identical to the physical path**, instruction for instruction. **[L]** |
| Magician 1st job skill ids | **not what the classic tree says.** `2001000` **Magic Guard**, `2001001` **Magic Armor**, `2001002` **Energy Bolt**, `2001003` **Magic Claw**. `2001004`/`2001005` **do not exist** in this build. **[L]**, `String.wz/Skill.img` |
| Magic Guard CTS bit | **97.** **[L]** — the client's own hit handler reads `secStat+0x614`, which is bit 97's value slot, and multiplies it by the damage / 100 |
| Magic Armor CTS bits | **86** (Weapon Def.) and **87** (Magic Def.). **[D]** — §7.3 gives the chain and names its blind spot |
| Value width / duration unit | unchanged from `research/buffs.md`: value **[I] `i16`**, duration **[D] milliseconds** |
| Magic Guard: client or server? | **The server must compute it.** The client computes the same number but **never writes HP** — `research/user-hit.md` §4.1–4.3 enumerated every write to `charStat+0x5b` and none is reachable from the hit path. Only `0x007C` moves the bars. **[L] + [D]** |

---

## 1. Finding the magic function, and the control that made it a measurement

`FUN_14025e000` — the physical damage-parameter builder — has **six call sites in five
functions** (`tools/callers.py`, all three modes, 0 tail jumps, 0 data pointers):

```
0x14025f380  0x1402617c6  0x140261c40  0x1411551e0  0x1411553e7
```

`0x14025f380` (3131 bytes) is the physical damage calculator. Its callers are
`0x1420b4710`, `0x1428c1fa0`, `0x1428cb4a0`, `0x1429a41e0`. **Every one of those also calls
`0x14025ffd0`** (plus a fifth, `0x1428cd6d0`), and in `FUN_1428c1fa0` — the attack builder
already listed in `research/msexe-attack-builder-1428c1fa0.txt` — the two calls are 298 bytes
apart on the two arms of one `if`, with **byte-identical argument setup** except the last
pointer:

```asm
1428c4397  mov  rax, [rbp - 0x30]      ; arg5, the attacker totals
1428c439b  mov  [rsp + 0x20], rax
1428c43a0  mov  r9,  [rbp + 0x90]      ; arg4, the character's stat object
1428c43a7  mov  r8,  [rbp - 0x28]
1428c43ab  mov  rcx, [rbp + 0x98]
1428c43b2  call 0x14025ffd0            ; MAGIC   (arg8 = rbp+0x2e0)
...
1428c44dc  call 0x14025f380            ; PHYSICAL(arg8 = rbp+0x340)
```

**[L]** That is the discriminator: same inputs, two functions, one `if`. `FUN_14025ffd0` reads
`statObj + 0x3c` — the slot `damage-formula.md` §1.5 identified as **INT** precisely because
`FUN_14025e000` *never touches it*. The function that was found by looking for the one stat the
physical builder ignores is the one that reads it.

---

## 2. The magic formula, in the shape `damage.rs` already uses

### 2.1 The core

```text
M     = (min(mastery, 10) / 10 + 0.1) * 0.8              # FUN_14025e840, SHARED with physical
u     = rand32 * 2^-32                                    # a fresh draw, see 2.4
Q     = INT*M + u * (INT - INT*M)      == uniform(INT*M, INT)
raw   = (skillMagic / 100.0) * MagicTotal * (1.0 + Q / 100.0)
```

Instruction for instruction (`0x14026053a`..`0x1402605d1`): **[L]**

```asm
140260540  movd     xmm2, [rsp+0x68]     ; INT, from getter(statObj+0x3c)
14026054a  movaps   xmm1, xmm2
14026054d  mulsd    xmm1, xmm6           ; xmm6 = M
   ... order the pair, then lo + u*(hi-lo) ...
1402605aa  movd     xmm6, [rsp+0x50]     ; the skill's magic damage number
1402605b4  divsd    xmm6, xmm9           ; xmm9 = 100.0   (0x143277e78)
1402605b9  movd     xmm0, [r13+8]        ; MagicTotal
1402605c3  mulsd    xmm6, xmm0
1402605c7  divsd    xmm2, xmm9           ; Q / 100
1402605cc  addsd    xmm2, xmm10          ; xmm10 = 1.0    (0x1434b95e0)
1402605d1  mulsd    xmm6, xmm2
```

`(skillMagic/100) * MagicTotal` and `MagicTotal * (skillMagic/100)` are the same number, so
this **is** meowdb's

```text
MIN = S * MagicTotal * (1 + TotalINT * M / 100)
MAX = S * MagicTotal * (1 + TotalINT     / 100)
```

with `S = skillMagic / 100`. The guide is right about magic, and now it is **[L]** rather than
a fan site. Note what is **absent**: there is **no second `uniform(0.8, 1.0)` roll**. Physical
has two independent rolls (`B` and `Q`); magic has one.

### 2.2 What happens to `raw` on its way to the screen

Every step below is **[L]** and every step is the **same code shape as the physical path** —
the two functions call the same helpers with the same constants, and I checked that by
listing both.

```text
1. defence     d = raw * 100 / (EffectiveMDEF + 100)        ; if (MDEF+100)==0 -> d = 0
2. element     d *= elementFactor(code)                     ; FUN_14025e8d0, table in 2.3
3. level gap   gap = mobLevel - playerLevel
               gap <= 0  -> divide by 1
               1..9      -> divide by (1 + gap^2 * 0.005)
               >= 10     -> divide by (1 + gap   * 0.05)
4. damage-up   if getter(totals+0x1b8) > 0:  d += d * FUN_14025e950(...)   ; a buff skill
5. critical    if trunc(u * 100) < totals[+0x20]:                          ; crit RATE, percent
                   d *= (totals[+0x24] + 100) / 100                        ; crit DAMAGE, percent
               the boolean is written to the hit record as a byte at [out-7]
6. finish      trunc(clamp(d, 1, 99999))                                   ; 0x1869f
```

Two things fall out that `research/damage-formula.md` §4 explicitly left as **[I]**:

* **§4's whole "rest of the outgoing model" is now [L] for both paths.** `100/(DEF+100)`, the
  element ladder, the level-gap scale with its `0.005`/`0.05`/`pow(.,2.0)`/`9`, and the
  `99999` cap are all in both `FUN_14025f380` and `FUN_14025ffd0` as literals. The addresses:
  `0x14025fccf`/`0x140260845` (`0.005`), `0x14025fcf8`/`0x14026081b` (`0.05`),
  `0x14025fceb`/`0x140260838` (`2.0` to `pow`), `0x14025f95b`/`0x14026095c` (`0x1869f`).
* **The critical rate and multiplier are per-character fields, not global constants.**
  `crates/world/src/damage.rs` has `CRIT_RATE = 0.05` and `CRIT_MULTIPLIER = 1.20` as
  constants; the client reads a **percent rate** from `totals+0x20` and a **percent damage
  bonus** from `totals+0x24`. `5` and `20` may well be the beginner values, but they are
  values, not constants, and a magician with `+critical` gear would move both. **[L]**

### 2.3 The element table, exact

`FUN_14025e8d0` is a bare 7-entry jump table on an `int`, no `.pdata` record of its own
(jump table at `0x14025e928`). **[L]**

| code | factor | | code | factor |
|---:|---:|---|---:|---:|
| 0 | 1.00 | | 4 | 0.25 |
| 1 | 0.00 | | 5 | 0.50 |
| 2 | 0.75 | | 6 | 1.50 |
| 3 | 1.25 | | >6 | 1.00 |

The seven values are exactly the guide's `0.00 / 0.25 / 0.50 / 0.75 / 1.00 / 1.25 / 1.50` —
and now they have their codes. The code comes from `FUN_140474fa0(mob, attrA, attrB)` for
magic and from `[req + …]` for physical.

### 2.4 The random source, because the unit matters

Both functions **pre-draw eleven `u32`s per call** into an 11-slot ring on the stack
(`[rbp+0x38 .. +0x64]` in magic, `[rbp+0xe8 .. +0x114]` in physical — 44 bytes each) with
`FUN_1407386b0`, and index it `i % 11` with an increment per consumer. **[L]**

```asm
1402603bd  movsd  xmm11, [rip+…]        ; 0x14327a9c8 = 2.3283064365386963e-10  == 2^-32
...
140260598  cvtsi2sd xmm0, rdx           ; rdx = a zero-extended u32
14026059d  mulsd    xmm0, xmm11         ; u = rand32 / 2^32   in [0, 1)
```

So `u` is a **half-open [0,1)** draw at 2^-32 resolution, not a `[0,1]`. A server-side
re-implementation that used an inclusive roll would sit one ulp above the client's maximum.

### 2.5 Effective magic defence

`FUN_14025e770(mob)` — **called only by the magic path** (`tools/callers.py`: two callers,
`FUN_14025ffd0` and `FUN_1402624a0`). Its physical twin is `FUN_14025e6c0` (four callers, all
physical). Same shape, different indices: **[L]**

```text
magic    : mdef = FUN_1404751c0(mob, 5) + trunc((FUN_1404751c0(mob, 6)/100 + 1) * [mob+0x2c])
physical : pdef = FUN_1404751c0(mob, 3) + trunc((FUN_1404751c0(mob, 4)/100 + 1) * [mob+0x28])
both     : clamped to >= 0 (cmovs), and both short-circuit to FUN_14047a1?0 when [mob+0x4c0] > 0
```

`[mob+0x28]`/`[mob+0x2c]` are the template's `PDDamage`/`MDDamage`; `FUN_1404751c0(mob, N)` is
the mob's temporary-stat getter, so index 3/4 are the flat and percentage **PDD** modifiers and
5/6 the **MDD** ones.

> **One asymmetry, unexplained, and worth writing down rather than smoothing over.** The
> physical helper has an extra early exit the magic one does not:
> `cmp dword [mob+0x40], 0 / jle …` — if `[mob+0x40] > 0` it **returns 0 defence**. I did not
> identify `+0x40`. It is not in the magic helper at all.

---

## 3. What carries over from `research/damage-formula.md`, and what does not

| piece of that file | magic? |
|---|---|
| `mastery_factor` = `(m/10 + 0.1) * 0.8`, `mastery` is 1..=10 | **shared, same function.** `FUN_14025e840` is called by the physical calculator at `0x14025f75a` and the magic one at `0x1402601cc` |
| `Scalars` — `roll_lo/roll_hi`, `stat_div`, `ap_div` | **does not apply.** They live in `FUN_14025e000`'s output struct and magic never calls it |
| `weapon_multiplier`, the 30..=47 table, Lucky Seven, bow-as-a-club | **does not apply** |
| `primary_and_secondary` (STR/DEX/LUK) | **does not apply.** Magic reads INT and nothing else |
| `after_defence`, `level_gap_scale`, `finish`, `DAMAGE_CAP` | **shared, and all four are now [L] instead of [I]** |
| element ladder | **shared**, and now [L] with codes (§2.3) |
| §3.2's `flag_b` is the crit flag | **structurally confirmed.** Both functions write the crit boolean as a byte at `hit_record - 7` (`0x14025fdd7`, `0x140260946`), which is where `AttackHit::flag_b` sits |
| §5 mob→player | untouched. Different function (`FUN_1428aa0a0`), and §8 below adds Magic Guard to it |

### 3.1 Two constants in `damage.rs` labelled [I] that are now [L]

`FUN_14087c130` is the attacker-totals builder (§7.3). Its first four writes are: **[L]**

```asm
14087c19e  eax = getter(statObj+0x24) ; sar eax,2   -> totals[+0x0c]     ; floor(STR/4)
14087c1b4  eax = getter(statObj+0x3c) ; sar eax,1   -> totals[+0x08]     ; floor(INT/2)
14087c1ce  eax = getter(statObj+0x3c) ; sar eax,2   -> totals[+0x14]     ; floor(INT/4)
14087c24e  trunc((2*Level + 1.2*DEX + 0.6*LUK)/2.5 + 5.0) -> totals[+0x18]
```

* `damage::wdef_from_strength` — `floor(STR/4)`, marked **[I]** — is **[L]**.
* `damage::mdef_from_intelligence` — `floor(INT/4)`, marked **[I]** — is **[L]**.
* `floor(INT/2)` is where **MagicTotal** starts, which settles the guide's decomposition.
* The four accuracy constants read `1.2` (`0x14327aa48`), `0.6` (`0x1432a4ae8`),
  `2.5` (`0x143286d28`) and `5.0` (`0x14327aa78`) — **meowdb's Beginner accuracy line,
  verbatim**, which `damage-formula.md` §7 recorded as [I] from a re-fetched web page.
  Every character on this server is a beginner, so this is the only accuracy line that could
  matter.

---

## 4. The two inputs, and how sure I am of each

### 4.1 `MagicTotal` = the attacker-totals field at `+0x08` — **[L]**

`arg5` to both damage functions is `user->vtable[0x30]()`, and `FUN_14087c130` builds it. The
per-hit multiply is `movd xmm0, [r13+8]` at `0x1402605b9`. Enumerating **every** memory access
to that argument inside each function (capstone operand walk, not a grep) gives a clean split:

```text
physical  [+0x00]  the whole bracket is multiplied by it   -> TotalWATK
          [+0x04]  divided by ap_div (50 / 150)            -> AttackPower
magic     [+0x08]  the whole product is multiplied by it   -> MagicTotal
both      [+0x20]  crit rate %          [+0x24]  crit damage %
```

### 4.2 The skill's magic damage number — **[D]**, and here is the chain

`[A + 0x5c]`, where `A` is the third qword of the object `FUN_140909d80(&out, skillId, level)`
fills — a **per-skill-level** lookup. The physical function reads `[A + 0x14c]` from the same
slot of the same object.

Four things point the same way and none of them is a listing of the WZ loader:

1. **The defaults differ the way they should.** The physical local is pre-set to `100.0`
   (`0x143277e78`) before the lookup and overwritten on success — a *percentage* whose neutral
   value is 100, i.e. a plain swing with no skill. The magic local is pre-set to **`0`**: a
   plain swing does no magic damage.
2. **This build's magic skills carry `mad` and no `damage`; physical skills carry `damage` and
   no `mad`.** Energy Bolt `2001002` is `mad: 90…130` across 20 levels with no `damage` node;
   Power Strike is `damage: 160…260` with no `mad`.
3. **The client's own tooltip calls `mad` the skill's attack number.** `String.wz/Skill.img`
   `2001002/h1` is `'MP -8, Basic Attack 90, Mastery level 1'` and level 1's `mad` is `90`;
   `2001003/h1` is `'MP -10, Basic Attack 45, …'` and its level-1 `mad` is `45`. **[L]**
4. It is the only per-level number Energy Bolt has that scales, and the drawn damage scales.

**Named blind spot:** I did **not** find the code that writes `A+0x5c` from the WZ. The one
`lea` of a wide `"mad"` literal in the image (`0x1403c9840`) is in `FUN_1403c8a40`, which is the
**consumable/item effect** parser — it writes `mad` to `[r14+0x138]`, `padRate` to `+0x16c`,
`crt` to `+0x14c`, and it is not the skill-level struct. So the identification above is
**[D]**, resting on defaults + WZ + the tooltip, not on the loader. `tools/xref.py` documents
exactly the blind spot that would hide the loader: a struct filled from a **table** of
`{name, offset}` pairs, or by an inlined literal copy, leaves no `lea` to find.

---

## 5. Worked numbers, so the next run has something to falsify

A level-10 Magician with **INT 40**, Magic Guard/Armor irrelevant, casting **Energy Bolt
level 1** (`mad 90`, `mastery 1`) at a template-2 snail (MDD 0, level 1, no element):

```text
M          = (1/10 + 0.1) * 0.8 = 0.16
MagicTotal = floor(40/2) + 0 equipment MATK = 20
Q          in [40*0.16, 40] = [6.4, 40]
raw        in [ (90/100)*20*(1+0.064),  (90/100)*20*(1+0.40) ]  =  [19.15, 25.20]
defence    x 100/(0+100) = x1        element x1        level gap: gap = 1-10 <= 0 -> x1
final      19 .. 25, and a crit multiplies by (100+critDamage)/100
```

At **mastery 10** the same cast gives `M = 0.88`, `Q in [35.2, 40]`, `raw in [24.34, 25.20]` —
the tooltip's *"The higher the Mastery, the lower the damage variation"* is exactly this term,
and it is a **distinguishable** on-screen outcome: the spread collapses as mastery rises.

---

## 6. What a magic `check_hit` would need

`crates/world/src/damage.rs` already has `check_hit`, `max_plausible_hit` and `HitVerdict`, and
`research/damage-formula.md` §9.1 says plainly they have **no caller anywhere** and should not
get one yet. The magic equivalent is *smaller* than the physical one, and for one specific
reason: **the two unknowns that gut the physical check do not exist here.**

| input | physical | magic |
|---|---|---|
| which **action** was played | **unknown** — an unexplained `0x00DF` header field | **irrelevant.** Magic has no action multiplier |
| which **skill** | **unknown** | **also unknown, and it is the whole formula.** `skillMagic` and `mastery` both come from it |
| weapon | known (equip slot 11) | **irrelevant** |
| stats | known | known — INT only |

So a magic ceiling is:

```rust
pub struct MagicAttacker { pub magic_total: u32, pub intelligence: u32, pub mastery: u32,
                           pub skill_magic_percent: u32 }

// max, before defence/element/level-gap (all of which only ever reduce) and before the crit
fn magic_max(a: &MagicAttacker, crit_damage_percent: u32) -> u64 {
    let raw = (a.skill_magic_percent as f64 / 100.0) * a.magic_total as f64
            * (1.0 + a.intelligence as f64 / 100.0);
    damage::finish(raw * (100.0 + crit_damage_percent as f64) / 100.0)
}
```

`mastery` never appears — it only moves the *minimum*, so it cannot raise a ceiling. `finish`,
`after_defence` and `level_gap_scale` are already in `damage.rs` and are shared.

**But it still cannot be armed today**, and the reason is worth stating rather than
implementing around: the server does not know the skill. Without it, `skill_magic_percent` has
no defensible value (a magician's plain swing is *physical*, so `100` is not a neutral default
here — the neutral default is `0`), and a ceiling built on a guessed skill is a validator that
can refuse a legitimate cast. `research/damage-formula.md` §11 already names the fix — **find
the skill and action fields in the `0x00DF` header** — and it buys the magic check outright,
because that is the *only* thing it is missing.

Two more constraints, both from `CLAUDE.md`:

* **`MagicTotal` is the client's number, and the server does not have it.** `floor(INT/2)` we
  can compute; equipment `incMAD` needs the column, and `gm-handbook/equips.txt` is missing
  `attackSpeed` and `attack` already (`damage-formula.md` §6.4) — check `incMAD` is present
  before assuming it is.
* **Never refuse the packet.** Log-only, exactly as §9.1 of that file says.

---

## 7. The two first-job Magician buffs

### 7.1 The ids in this build are not the classic ids

`Skill_000.wz/200.img` has **six** skills, and `String.wz/Skill.img` names them. **[L]**

| id | name | `info.type` | levels | what a level carries |
|---|---|---|---|---|
| `2000000` | Improved MP Recovery | 50 | 10 | `x`, `y` |
| `2000001` | Max MP Increase | 50 | 10 | `mmpR` 10..19; requires `2000000` lv3 |
| **`2001000`** | **Magic Guard** | **10** | **15** | `mpCon` 8..12, **`x` 30..80**, **no `time`, no `cooltime`** |
| **`2001001`** | **Magic Armor** | **10** | **20** | `mpCon` 8..16, **`time` 300..600**, **`indiePdd` 40..120**, **`indieMdd` 40..120**; requires `2001000` lv3 |
| `2001002` | Energy Bolt | 2 | 20 | `mad` 90..130, `mastery` 1..10, `attackCount` 1 |
| `2001003` | Magic Claw | 1 | 20 | `mad` 45..65, `mastery` 1..10, `attackCount` 2; requires `2001002` lv1 |

**`2001004` and `2001005` do not exist here.** The classic tree puts Magic Guard at `2001002`
and Magic Armor at `2001003`; in this build those two ids are **Energy Bolt and Magic Claw**.
Anything that hard-codes the classic numbers will buff the wrong skill, and both wrong targets
are attacks, so nothing would visibly happen.

The client's own tooltips: **[L]**

```text
2001000  h1   'MP -8; Replace 30% of HP damage with MP while active'
         desc 'Replaces a portion of incoming damage with MP instead of HP. If MP reaches 0,
               all damage is received as HP damage. The effect is activated when used and
               deactivated when used again.'
2001001  h1   'MP -8; Weapon Def. +40, Magic Def. +40 for 300 sec'
```

**Magic Guard is a toggle**, not a timed buff — the WZ has no `time` node at any of its 15
levels and the description says so in words. Magic Armor is an ordinary timed buff with the
same `processtype 6` and `additional_process {0: 114}` as Nimble Feet, so it is structurally
the packet `research/buffs.md` §7.1 already builds.

### 7.2 The instrument that names a CTS bit, and its controls

`.rdata` `0x143303fc8..0x1433060xx` holds a run of **plain ASCII temporary-stat names**
(`MagicGuard`, `IronWill`, `Booster`, `PowerGuard`, …). Nothing points at them from data — they
are `lea`'d, by **four enormous unrolled functions**: `FUN_140a5a4b0` (274 309 bytes),
`FUN_140afc2a0` (250 192), `FUN_140b39400` (395 923), `FUN_140bbf2d0` (435 468). Each is a
linear chain of blocks of the shape

```asm
140afe4b0  mov  dword [rsp+0x6c0], 0x61     ; the CTS index, 97
140afe4c9  cmp  dword [rsp+0x3258], 0 / jl  ; skip if < 0
140afe4d3  cmp  dword [rsp+0x6c0], 0x52 / jg ; skip unless > 82
140afe517  lea  r8, "MagicGuard"            ; format "%s", compare against an input string
```

Pairing each name `lea` with the nearest preceding `mov dword [rsp+…], imm` in a 25-instruction
window gives **323 names over 2 666 references**, with **zero disagreements across the four
functions**. Four controls before believing it:

1. **Self-consistency.** 323 names, 4 independent functions, 0 conflicts.
2. **Every one of the 323 indices is present in `FUN_140a165f0`'s bit census**
   (`research/msexe-secondarystat-140a165f0.txt`, 474 blocks / 410 distinct bits). **323 of
   323**, no exceptions.
3. **It discriminates.** Shifting the whole name set by ±1..6 drops it to 308–317 of 323, and
   only shift 0 is perfect. The census is 97.6% dense over 83..502, so a chance perfect hit is
   ~4 in 10 000.
4. **Both ends agree.** The naming chain refuses any index `<= 82`; the decoder's lowest bit is
   **83**. The naming chain's highest is **502**; the decoder's highest is **502**.

### 7.3 Magic Guard is CTS bit **97** — [L], and it does not rest on the table above

The table says `97 -> MagicGuard`. That would be [D]. It is **[L]**, because the client's own
hit handler reads the bit and does the thing the skill is named for.

**Step 1 [L].** Every CTS bit's value/reason/expire offsets, recovered by disassembling the
per-bit setter functions the census names. **The control is bit 92**: `research/buffs.md` §6
established `0x5cc / 0x5d8 / 0x5e4` for Speed by a completely different route (a string id and
a guard function), and this walk reproduces it exactly.

```text
bit   value   checksum  reason   expire        bit   value   reason  expire
 83   0x398    0x3a0    0x3a4    0x3b0          92   0x5cc   0x5d8   0x5e4   <- CONTROL, Speed
 84   0x3c8    0x3d0    0x3d4    0x3e0          93   0x5f0   0x5fc   0x608
 85   0x404    0x40c    0x410    0x41c          94   0x728   0x734   0x740
 86   0x440    0x448    0x44c    0x458          95   0x74c   0x758   0x764
 87   0x47c    0x484    0x488    0x494          96   0x6bc   0x6c8   0x6d4  Booster
 88   0x4b8    0x4c0    0x4c4    0x4d0          97   0x614   0x620   0x62c  MagicGuard
 89   0x4f4    0x4fc    0x500    0x50c          98   0x638   0x644   0x650  IronWill
 90   0x530    0x538    0x53c    0x548          99   0x65c   0x668   0x674
 91   0x56c    0x574    0x578    0x584         100   0x6e0   0x6ec   0x6f8  PowerGuard
```

**Step 2 [L].** A byte scan for `lea reg, [reg + 0x614]` over the whole `.text`, every ModRM
form, each candidate re-disassembled to confirm an instruction boundary (the control, `0x5cc`,
returns its known readers including `FUN_1429755a0`'s guard). One of the hits is inside
**`FUN_1428aa0a0`** — which `research/damage-number-draw.md` and `research/user-hit.md` already
identify as the client's **own hit handler**, the function that draws the damage number over
the player's head and builds the `0x00E5` hit report:

```asm
1428ac938  mov  edx, [r13 + 0x61c]        ; the checksum of bit 97's value triple
1428ac93f  lea  rcx, [r13 + 0x614]        ; bit 97's value
1428ac946  call 0x1401ba9d0               ; the de-obfuscating getter
1428ac94b  test eax, eax
1428ac94d  je   0x1428ac9cb               ; zero -> no Magic Guard -> skip the whole block
1428ac94f  ... getter(r13 + 0xe44) ...    ; a second stat; picks which of two arms runs
1428ac993  call 0x1401ba9d0               ; re-read bit 97's value
1428ac998  mov  ecx, eax
1428ac99a  imul ecx, esi                  ; value * damage
1428ac99d  mov  eax, 0x51eb851f / imul / sar edx,5 / …   ; the signed "/ 100" idiom
1428ac9b4  lea  rcx, [rax + 0x73]         ; a second stat field, read as a cap
1428ac9c0  cmp  eax, ebx / cmovl ebx, eax ; clamp the MP loss to it
1428ac9c5  mov  [rbp - 0x70], ebx
```

**The value at bit 97 is multiplied by damage and divided by 100, then clamped.** That is a
percentage, and it is `Skill.wz` `2001000/level/N/x` — 30 at level 1, exactly what the tooltip
says. Two halves found by different routes meeting on one offset: the name table says
`97 = MagicGuard`, and the client multiplies bit 97's value by incoming damage and clamps it to
a second field. **[L]**

The clamp target `[rax + 0x73]` is **[D] the character's current MP** — `research/user-hit.md`
§4 established `charStat + 0x5b` is HP with a 12-byte obfuscated-triple stride, which puts
`+0x67`, `+0x73`, `+0x7f` on the next three slots — and the skill description says *"If MP
reaches 0, all damage is received as HP damage."*

**Bit 97 carries no extras.** Its census row is a standard `u32,u16,u32,u32` 87-byte block, and
97 is **not** among the 67 conditional-extras blocks. So a Magic Guard `0x007D` has **exactly
the shape of the Nimble Feet one** — mask, `i16 value`, `u32 reason`, `u32 duration`, zero tail.

### 7.4 Magic Armor is CTS **86** and **87** — [D], with the chain and the blind spot

This one is not [L], and the reason is a real complication rather than a gap in the search.

**Magic Armor's WZ keys are `indiePdd` and `indieMdd`, and "Indie" is a different index space.**
`.rdata` `0x14327ce58..0x14327d178` holds a **second** name run — `IndieWAT`, `IndiePAD`,
`IndieMAD`, `IndiePDD`, `IndieMDD`, `IndieACC`, … — and the same extraction over its four
referencing functions gives **57 names indexed 0..55**, with `IndieMAD = 1`, `IndiePDD = 2`,
`IndieMDD = 3`, `IndieACC = 4`, `IndieEVA = 5`. **[L]** That numbering is not the CTS numbering
(which starts at 83), none of those names appears in the CTS table, and **`FUN_140a165f0`
decodes no bit below 83 and contains no list-shaped block that could carry them** — its 474
blocks are 407 standard `u32,u16,u32,u32` stats plus 67 conditional extras, and I enumerated
every read shape rather than looking for one.

So the Indie stats do not appear to ride in `0x007D` at all, and the useful question becomes
**which plain CTS bit moves the client's Weapon Def. and Magic Def.**

`FUN_14087c130` answers it. It is the attacker-totals builder from §3.1, and after seeding the
totals from the base stats it reads **nine consecutive CTS values, 83..91, in index order**, and
adds them to the totals **in order**: **[L]**

```asm
14087cf34  getter(secStat+0x398)  -> ebx        (CTS 83)   ... add [rbx+0x00], …+ebx
14087cf49  getter(secStat+0x3c8)  -> edi        (CTS 84)   ... add [rbx+0x04], …+edi
14087cf5e  getter(secStat+0x404)  -> esi        (CTS 85)   ... add [rbx+0x08], …+esi
14087cf73  getter(secStat+0x440)  -> r14d       (CTS 86)   ... add [rbx+0x0c], …+r14d
14087cf89  getter(secStat+0x47c)  -> r15d       (CTS 87)   ... add [rbx+0x14], …+r15d
14087cf9f  getter(secStat+0x4b8)  -> r12d       (CTS 88)   ... add [rbx+0x18], …+r12d
14087cfb5  getter(secStat+0x4f4)  -> [rbp-0x74] (CTS 89)   ... add [rbx+0x1c], …
14087cfcb  getter(secStat+0x530)  -> r13d       (CTS 90)   ... add [rbx+0x20], …+r13d
14087cfe8  getter(secStat+0x56c)  -> r8d        (CTS 91)   ... add [rbx+0x24], …+r8d
```

Cross that with the four seeded fields from §3.1 and the two damage functions' own use of the
same struct, and every column is pinned by something other than the guess:

| totals | seeded with | consumed as | CTS |
|---|---|---|---:|
| `+0x00` | — | the whole physical bracket is multiplied by it | **83** = PAD |
| `+0x04` | — | divided by `ap_div` (50/150) | **84** = AttackPower |
| `+0x08` | `floor(INT/2)` | the whole magic product is multiplied by it | **85** = MAD |
| `+0x0c` | `floor(STR/4)` | — | **86** = **PDD / Weapon Def.** |
| `+0x10` | — | — | (no CTS) |
| `+0x14` | `floor(INT/4)` | — | **87** = **MDD / Magic Def.** |
| `+0x18` | the accuracy line | — | **88** = ACC |
| `+0x1c` | — | — | **89** = EVA |
| `+0x20` | — | crit **rate** percent | **90** |
| `+0x24` | — | crit **damage** percent | **91** |

`floor(STR/4)` is the guide's `WDEF` and `floor(INT/4)` is its `MDEF`; the CTS values added on
top of them are therefore the temporary Weapon Def. and Magic Def. That reads the same way the
tooltip does — *"Weapon Def. +40, Magic Def. +40"*.

> **Named blind spot, quoted so it can be acted on.** Nothing I found says the *server* is
> supposed to grant Magic Armor as CTS 86/87 rather than as Indie 2/3 through some mechanism I
> have not located. What I established is: (a) the skill's WZ keys are `indiePdd`/`indieMdd`;
> (b) the Indie names are a separate 0..55 space that the `0x007D` decoder shows no sign of
> carrying; (c) CTS 86 and 87 are the bits this client adds to its own Weapon Def. and Magic
> Def. totals. (c) is [L]. Choosing 86/87 *because* of (b) is [D], and **one run settles it**
> — §9.3.

### 7.5 Value width and duration, unchanged

Both from `research/buffs.md` and neither is new here:

* **Value width is [I] `i16`.** The deciding constant is at `0x143ac37e0` in Themida-packed
  `.data`, so no static read can settle it. Magic Guard's value is 30..80 and Magic Armor's is
  40..120; both fit an `i16` and both fit a `u32`, so **neither skill discriminates the width
  on its own** — the run that already answered it for Nimble Feet is the evidence.
* **Duration is milliseconds, [D]** — three agreeing readings, one of them a wire measurement.
  `Skill.wz` `time` is **seconds**: Magic Armor level 1 is `300` → `300_000`.
* **Magic Guard has no `time` at all.** See §9.2.

---

## 8. Magic Guard: the server has to compute the split

**The client already computes the same number and cannot apply it.** Both halves matter.

* **It computes it.** §7.3: `mpLoss = trunc(magicGuardPercent * damage / 100)`, clamped to
  current MP, inside `FUN_1428aa0a0`. **[L]**
* **It does not move HP.** `research/user-hit.md` §4.1–4.3 enumerated **every** write to
  `charStat+0x5b` in the whole image — 18 sites, 8 real, positive control found — then closed
  the `lea`-handoff blind spot with **all 196** `lea reg,[reg+0x5b]` sites, and then walked the
  hit path to depth 8 over 5 716 functions with three reached controls. **No HP writer is
  reachable from the hit path.** The bar moves only on `0x007C`. **[L]**
* **This server owns incoming damage.** `Session::on_user_hit`
  (`crates/world/src/session/combat.rs:506`) discards the client's number when it has a
  template, computes its own with `damage::incoming_damage`, subtracts it from `chr.hp` and
  sends `0x007C`.

So Magic Guard is **not** a client-side consequence of the stat bit. Setting bit 97 makes the
client *display* the buff and compute a number for its own report; the HP and MP the player
actually sees are whatever `0x007C` says. **If the server does not split the damage, Magic
Guard does nothing observable except an icon.**

What the server must do, in `on_user_hit`, is the client's own arithmetic:

```text
mp_loss = trunc(magic_guard_percent * applied / 100)
mp_loss = min(mp_loss, chr.mp)          # "If MP reaches 0, all damage is received as HP damage"
hp_loss = applied - mp_loss
```

and then send **HP and MP together** in one `0x007C`. `net::stats::StatChange::hp_only` is what
that handler uses today; a magic-guarded hit needs both bits or the MP bar will not move.

> **The rule this is an instance of.** `CLAUDE.md`: *"Every effect hangs off the transition,
> not off the request."* The Heena bug was a payout outside the match on the store's answer.
> Here the shape to avoid is the mirror: computing `hp_loss` and `mp_loss` in one place and
> then persisting only one of them. `chr.hp` and `chr.mp` must be written and saved together,
> or a crash between them leaves a character who paid MP and kept HP.

---

## 9. WIRE IT LIKE THIS

Nothing in this file is wired. `crates/net/src/buff.rs::buff_level` returns `None` for
everything except Nimble Feet (`1002`), so both Magician buffs are dropped today.

### 9.1 Outgoing magic damage — DO NOT WIRE, for the same reason as physical

`research/damage-formula.md` §9.1 says the physical formula's only server-side use is
validation and that validation needs the action and the skill, neither of which is parsed.
Magic needs **only the skill** (§6) — a smaller gap, and the same gap. Until the `0x00DF`
header's skill field is found, a magic ceiling has no defensible `skill_magic_percent`.

If it is wired anyway: **log only, never refuse.**

### 9.2 Magic Guard `2001000` — the packet, and the one thing it needs decided

```text
opcode 0x007D, body 198 bytes (mask 124 + stat 10 + TAIL_LEN 64)

  mask bytes 12..15 = 00 00 00 40      <- bit 97 and nothing else
```

**Get that from `net::buff::stat_mask(&[97])`, not from a literal.** The arithmetic is word
`97 >> 5 = 3` — bytes 12..15 as a little-endian `u32` — and `1 << (31 - (97 & 31))` =
`1 << 30` = `0x40000000`, which lands as `00 00 00 40`. `stat_mask` already does exactly this
and has a round-trip test over every legal bit; the "obvious" `1 << (i & 31)` would set bit 66
and the client would grant it without complaint. Magic Armor's pair lands in word 2:
`stat_mask(&[86, 87])` puts `0x00000300` in bytes 8..11.

```text
value    = i16  Skill.wz 2001000/level/<lv>/x      (30, 33, 36, … 80)
reason   = u32  2001000
duration = u32  ???  <- Magic Guard has NO `time` node. See below.
tail     = 64 zero bytes
```

**The duration is a decision, not a lookup.** Magic Guard is a toggle: 15 levels, no `time`, no
`cooltime`, and the description says *"activated when used and deactivated when used again."*
Three facts constrain the choice, all from `research/buffs-underflow.md`:

* the client's `tExpire` drives **only the icon animation**;
* at natural expiry the client sends **nothing** and just flashes the icon;
* **the server owns removal** — `0x007E` is mandatory.

So send a large duration (the icon then counts down from a silly number, which is visible and
ugly) **or** treat the second `0x013C` for `2001000` as the off switch and send `0x007E` for
bit 97 then. The second is what the skill actually is. Either way the *server's* record of
"Magic Guard is on" is what `on_user_hit` must consult — not the client's.

`0x013F` (right-click the icon) already parses, and `parse_skill_cancel` returns the bits; a
cancel carrying bit 97 is the player switching it off.

### 9.3 Magic Armor `2001001` — an ordinary timed buff, two bits

```text
stats, in ASCENDING bit order (temporary_stat_set sorts them, do not rely on the caller):
  bit 86  value i16 = indiePdd (40, 44, 48, … 120)   reason 2001001   duration_ms = time*1000
  bit 87  value i16 = indieMdd (identical values)     reason 2001001   duration_ms = time*1000

body = 124 mask + 2 x 10 + 64 tail = 208 bytes
```

Neither 86 nor 87 is among the 67 conditional-extras bits, so no per-stat extras.
`temporary_stat_set` already emits ascending bit order — `research/buffs.md` §5.2's walk order.

**This is the one that needs a run**, and it is a genuine experiment because §7.4 is [D]:

| what the owner sees | what it means | next |
|---|---|---|
| icon appears, counts 300 s, and the **stat window's W. Def and M. Def both rise by 40** | 86 and 87 are PDD and MDD. §7.4 becomes [L] | wire it; add the CTS bonus to `incoming_damage_for`'s `wdef` |
| icon appears and counts down, **no stat window change** | the bits are wrong, or the client shows only equipment in that field | sweep 83..91 one bit per cast and watch which line moves; 83 should move Attack, which is a free positive control |
| **only one** of the two lines moves | the pair is off by one — the run names which | shift the other bit and re-test |
| client dies | copy `world.log`, `client-patched\maplecw-hook.log` and `client-exit.log` into `research/fixtures/` **first**, then count `0x007D` in `world.log` against the hook log's dispatch lines |

**One variant per launch.** Magic Armor first: it is a plain timed buff whose whole outcome is
readable off the stat window, and it needs no change to `on_user_hit`. Magic Guard second,
because its visible effect requires the §8 server change and testing both at once puts two
variables in one run.

### 9.4 Corrections to files this agent does not own

* **`crates/world/src/damage.rs`** — `wdef_from_strength` and `mdef_from_intelligence` are
  marked **[I]**; both are **[L]** (§3.1). `CRIT_RATE` and `CRIT_MULTIPLIER` are documented as
  constants; the client reads them per character from `totals+0x20` / `totals+0x24` (§2.2).
* **`crates/world/src/damage.rs`** — `after_defence`, `level_gap_scale`, `finish` and
  `DAMAGE_CAP` carry §4's `[I]` label from `damage-formula.md`; all four are now **[L]**, in
  both damage functions, with addresses in §2.2.
* **`research/damage-formula.md` §11** — the "Magic damage" bullet can be struck; §4 can be
  relabelled.
* **`crates/net/src/buff.rs`** — `buff_level`'s doc block explains why it is a literal table
  of three rows. It is now a table of three skills (`1002`, `2001000`, `2001001`) with 3, 15
  and 20 levels; that is worth a WZ read rather than 38 more literals.

---

## 10. What I could NOT establish

Each names the blind spot that produced it.

* **That `[skillLevel + 0x5c]` is the WZ `mad` node.** §4.2. Four independent things point at
  it and none is the loader. The one wide `"mad"` literal in the image belongs to the
  **consumable** parser, and `tools/xref.py`'s documented blind spot — a struct filled from a
  `{name, offset}` table, or by an inlined literal copy — is exactly what would hide the real
  one. **[D], not [L].**
* **Whether Magic Armor should be granted as CTS 86/87 or through an Indie mechanism I did not
  find.** §7.4. What I can say is that the `0x007D` decoder has no bit below 83 and no
  list-shaped block, and that CTS 86/87 are what this client adds to its own Weapon Def. and
  Magic Def. totals. The run in §9.3 decides it.
* **The names of CTS 83..95.** Only 96..502 are named by the client, and only because the four
  naming functions refuse indices `<= 82`. 83..91 are pinned by *role* (§7.4) rather than by
  name; **92 is Speed by an independent [L] route**; 93, 94 and 95 are unassigned by anything
  here. 93 sits immediately before 97 in memory and is a plausible Jump; that is a guess and is
  labelled as one.
* **`[mob + 0x40]`**, the early exit that zeroes *physical* defence and has no magic
  counterpart (§2.5). Read and not followed.
* **`[totals + 0x10]`** — the one field in the block that no CTS feeds (§7.4).
* **`[secStat + 0xe44]`** — the second stat Magic Guard's block reads, which selects between an
  arm that clamps the MP loss to current MP and one that does not. Almost certainly an
  "infinite MP" state; not followed.
* **Where the client's computed `mpLoss` goes.** It is written to `[rbp-0x70]` in
  `FUN_1428aa0a0` and the local is clobbered ~430 bytes later; I did not identify the consumer,
  so I do **not** claim it reaches the `0x00E5` hit report. It does not matter for the server —
  §8 — but "the client sends its MP loss" is a claim I am not making.
* **Whether `gm-handbook/equips.txt` carries `incMAD`.** Needed for `MagicTotal`, and
  `damage-formula.md` §6.4 already found two columns missing from that generator. Not checked.
* **Anything in `.themida`** — 20 MB of virtual address space with `SizeOfRawData` 0. Every
  negative above is "not in the 56 MB that exists on disk", the same standing blind spot
  `damage-formula.md` §1.1 names.

---

## 11. Instrument controls, run before anything above was believed

* `python tools/listing.py 0x140304100 | grep READ` → the documented reads at `140304138 raw`,
  `140304144 u8`, `140304183 u8`, then the `u16` run. **Passes.** (`tools/reads.py` in the repo
  contains `0x1406e8fb0` and `0x142d23ef0`; every command below was run with the **repo** as the
  working directory, per `CLAUDE.md`'s scratchpad-shadowing rule.)
* `python tools/callers.py 0x1402fa9a0` → **96 call sites in 15 functions, 43 of them in
  `0x140304b20`**, the documented positive control. **Passes**, and every `callers.py` result
  quoted above reports all three modes (`call`, tail `jmp`, data pointer).
* `python tools/xref.py --string "UI/Login.img"` → 67 copies. **Passes.**
* `python tools/dump_stringids.py --id 1331` → `'[Welcome] Welcome to MapleStory!!'`.
  **Passes.**
* **The CTS offset walk was checked against a value found by a different method.** Bit 92 comes
  out as `0x5cc / 0x5d8 / 0x5e4`, which `research/buffs.md` §6 derived from a string id and a
  guard function with no setter disassembly involved. **Agrees.**
* **The name-index extraction was checked four ways** before use — §7.2, including a shift test
  that discriminates and a boundary that two unrelated artefacts agree on.
* **The `lea [reg + 0x614]` byte scan was checked on `0x5cc` first**, and returned the readers
  a Speed scan must return. A scan that could not find Speed would not have been trusted to
  find Magic Guard.
* **`research/msexe-secondarystat-140a165f0.txt` was re-parsed by read shape, not filtered.**
  474 blocks: 407 `u32,u16,u32,u32`, 41 bare `u32`, 14 `u32,u32`, and nine other shapes, all
  enumerated and printed before any claim about "no list-shaped block" was made.
