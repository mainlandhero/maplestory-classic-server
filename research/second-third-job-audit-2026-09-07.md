# The second- and third-job skills: what the server does with each of the 149

The owner, 2026-09-07: *"take a look at all of the classes' 2nd and 3rd job skills, make sure that
most if not all of them are implemented correctly."*

No client run went into this. Everything is `gm-handbook/skills.txt` (generated from this
client's own `Skill.wz` and `String.wz` by `tools/dump_skills.py`), the client's 408-name
temporary-stat table (`research/first-job-buffs.md` Appendix A), its `0x007D` decoder census
(`research/msexe-secondarystat-140a165f0.txt`), and `crates/`. Tags: **[L]** read off this
client's data or listing, **[D]** derived, **[I]** inferred.

**Nothing here authenticates.** The game socket carries no credentials; a cast is honoured on
the say-so of whoever holds the socket, checked only against the level the database says that
character has.

**Nothing here has been on a screen.** Every claim below is a unit test against the client's own
numbers; `tools/test-server.ps1` T19 says what each screen outcome would mean.

---

## 0. The answer in one table

The census is **149 skills - 76 second-job, 73 third-job** - across twenty books. Enumerated
from the `job` column of the generated file, not from memory; there is no fifth branch and no
Pirate in this client. `[L]`

| what the server does | skills | how |
|---|---:|---|
| **grants a temporary stat**, with its server half where the number is the server's | 43 | §2, §3 |
| **an attack**: MP/HP/mesos/arrows from its own row, the client's damage applied | 38 | §1 |
| **Heal** - HP now, to the caster and the party on the field | 1 | §3 |
| **cost only** - the client's table prices it and the server produces nothing | 16 | §4 |
| **a mob debuff** - cost only, and the status is not sent: no mob-stat packet is decoded | 11 | §5 |
| **passive, the client's own arithmetic** | 18 | §6 |
| **passive with a server half, built** (MP Eater x3, Improved MP Recovery x2) | 5 | §6 |
| **passive with a server half, NOT built** | 4 | §6 |
| **hidden** (`invisible = 1`), never granted | 13 | §7 |

Those sum to 149. The per-skill table in §9 is the record, generated from the census by the
same rules; a row it could not classify would print as OPEN, and none does.

So: **most now do the correct thing on the server, and every one that does not says so in its
own row.** The largest thing that is not built is one packet, not many features: the mob-side
temporary stat (§5).

## 1. Every cast now pays, and attacks take what their own row says

### 1.1 The `0x013C` path used to keep the MP

Until today a `0x013C` for any skill outside the three buff tables was answered with *"This
server does not grant skill N's effect yet"* and **nothing was spent**. The client had already
spent it locally, so the next `0x007C` carrying the MP field handed it back - the same
stale-total bug the attack path fixed on 2026-08-28, on a second path. Teleport, Flash Jump,
every Booster, every summon, Mystic Door: all of them.

`Session::on_skill_use` now prices every cast from the client's own row: `mpCon`, `hpCon`
(ten Boosters cost 30 HP as well as 30 MP - **[L]**, the tooltip's *"HP -30; MP -30"* - and no
buff table ever carried an HP cost), and `itemCon`/`itemConNo` (Magic Rock `4006000` for Spell
Booster, Mystic Door and Meso Saver; Summoning Rock `4006001` for Shadow Partner and the four
summons). **Items are taken only when something was produced** - a rock for a summon this
server cannot show would be an item gone for nothing; the MP still goes, because the client's
copy already did. HP floors at 1, as Slash Blast's does. `[L]` for every number.

### 1.2 Arrows and stars, per row

`spend_attack_arrows` consulted `firstjob::server_obligation`, which knows the 24 first-job
skills and answers `None` for everything else - which fell to *"one per shot"*. Now, outside
the 24, the row decides, in the order the columns override one another:

| column | rule | skills | standing |
|---|---|---|---|
| `noBulletConsume 1` | none | the three hidden hits `3101005`, `3111006`, `3211006` | `[L]` |
| `bulletConsume n` | `n` | Arrow Rain / Arrow Eruption 8..4, Avenger 4, Inferno, Blizzard, Mortal Blow 1 | `[L]` |
| `bulletCount n`, no consume column | `n` on the shoot opcode | Strafe 3..4, Iron Arrow 1, Arrow Bomb 1 | `[I]` - the owner's "depending on the attack amount", the reading Lucky Seven already has |
| neither | one on the shoot opcode | | `[I]` |

**Soul Arrow** (`3101003`, `3201003`) is a grant (§2) and, while held, `spend_attack_arrows`
takes nothing at all - the client stops counting its own arrows down under CTS 104, so a server
that kept taking them would drift by one per shot. `[D]`

### 1.3 The other costs

* **Shadow Meso** (`4111003`) throws mesos - `moneyCon` 200..500 - and the server takes them
  now, never refusing (a short purse pays what it has and says so). `[L]`
* **Element Amplification** raises every MP cost by `x`% while held - *"MP cost increased to
  120%"*, `x = 20` - on both the cast path and the attack path. `[L]` for the number; `[I]`
  that it applies to every skill rather than only the spells (the tooltip does not qualify).
  The first version read `x` as the multiplier and scaled nothing; a unit test caught it.
  `CLAUDE.md`'s *the unit, not the arithmetic*, again.

### 1.4 What an attack's damage is

The client's, applied as sent - unchanged. `damage.rs`/`magic.rs` are validators and are not
wired to refuse, which is a standing decision (`STATUS.md` J). So masteries, criticals, Element
Amplification's damage half, the Charges' element and Shadow Partner's extra lines are all the
client's arithmetic and arrive in the attack packet.

## 2. Thirty-three more buffs are granted, and how each bit was chosen

`table_buff_level` already built a buff from any row whose grants were flat `indie*` columns.
Two things were added.

**Three more `indie*` columns.** `indieAcc -> 88`, `indieEva -> 89` (Bless; the same bits Focus
uses, `[L]`/`[D]` respectively) and `indieMhpR -> 94 MaxHP` (Hyper Body; the column is a ratio,
and 94 is the percent stat where `214 IncMaxHP` is the later flat one - `[D]`).

**A table for the buffs whose effect is not a column at all** - `crates/world/src/advbuffs.rs`.
Twenty-nine skills carry their effect in `x` or in nothing: a Booster is faster swings, Soul
Arrow is free shots, a Charge is an element on the blade, and the client switches each on when
it sees the bit. For those the generated table cannot say which bit; the name table does:

| bit | name | skills | value | standing |
|---:|---|---|---|---|
| 96 | Booster | the ten weapon boosters | `x` = `-2` stages | **[L]** name - a positive control of the name table |
| 100 | PowerGuard | 1111005, 1211005 | `x` percent | **[L]** name - a positive control |
| 101 | FinalAttack | the eight Final Attack toggles | level | [D] |
| 103 | Invincible | 2301002 | `x` percent | [D] |
| 104 | SoulArrow | 3101003, 3201003 | level | [D] |
| 105 | DragonBlood | 1311004 (+ its `indiePad` on 84) | level | [D] |
| 106 | WeaponElemCharge | 1211001..3 | level | [D] |
| 107 | ComboAttack | 1111000 | 1, then +1 per hit | [D]; the decoder reads **4 extra bytes** for this bit, out of the tail's slack |
| 108 | ElementAmplification | 2111000, 2211000 | level | [D] |
| 109 | SpellBooster | 2111005, 2211005 | `x` stages | [D] |
| 110 | HolySymbol | 2311002 | `x` percent | [D] |
| 112 | MesoGuard | 4211000 | `x` percent | [D] |
| 113 | ShadowPartner | 4111001 | level | [D] |

**What [D] means for every one of them, stated once**: the re-derived 408-name table names the
bit (zero conflicts, five controls reproduced), and the decoder census gives it the standard
87-instruction block, `reads=u32,u16,u32,u32`, `occ1` - byte-identical in shape to bit 92,
which a client has run. **No reader that consumes the slot has been traced for any of them.**
The client's own use of the stat is the run's to show, and T19 lists the outcome per bit.

**The value conventions** are three, and the census is why: `x` where the tooltip gives it a
unit the client can use directly (stages, percents); the **level** for the flags, which is the
reference-server convention and matches the one measured flag reader here (Dark Sight tests for
non-zero); and `1` for Combo, whose value is the orb count plus one. `[I]` for the latter two.

**Toggles.** Every `processtype 113` skill (`[D]`, sixteen-for-sixteen in the archive) is held
until cast again or right-clicked. **This exposed a latent bug**: `grant_buff_with_tail` recorded
a toggle's expiry as `now + 0`, and `buff_tick` clears anything at or past its expiry, so
**Magic Guard has been going off on the next pass of the session loop** since it was written,
`0x007E` and all. The one Magic Guard test grants and hits at the same instant and never saw a
tick. A toggle now has no expiry, and a test ticks an hour past a Magic Guard cast and finds it
held. `[L]` for the code.

## 3. The server halves - where granting the bit would only buy an icon

`session::buff::magic_guard_percent`'s sentence - *"setting the bit buys an icon, and the
arithmetic that makes the buff mean something is the server's"* - applies to nine of the new
grants. All nine are built, all against numbers the server owns:

| buff | the server's half | where | test |
|---|---|---|---|
| Hyper Body | raises the HP ceiling by the held percent, for as long as it is held; party members' own sessions do the same for their own copy | `session::pools` | `hyper_body_raises_the_servers_hp_ceiling_only_while_held` |
| Power Guard | `x`% of a hit never lands and goes to the mob that hit us, through the same body a swing uses (`deal_to_mob`, extracted from `on_attack` so the two cannot drift) | `on_user_hit` | `power_guard_reflects_its_share_onto_the_mob` |
| Invincible | `x`% of a hit ignored | `on_user_hit` | `invincible_and_meso_guard_take_their_shares_off_a_hit` |
| Meso Guard | `x`% blocked, paid at `y`% of the blocked amount in mesos; **nothing blocked when the purse cannot pay** (the client's own gate) | `on_user_hit` | same |
| Holy Symbol | `x`% more experience a kill (self only - no rectangle in this client's data) | `exp_for_kill` | `holy_symbol_adds_its_percent_to_a_kills_experience` |
| Element Amplification | every MP cost x `(100 + x)`% | `buff::amplified_mp` | `element_amplification_raises_every_mp_cost_but_its_own` |
| Dragon Blood | `x` HP every `y` seconds, floored at 1 | `buff::dragon_blood_tick` | `dragon_blood_drains_on_its_own_clock_and_never_kills` |
| Combo Attack | +1 per landed swing up to `y + 1`; Coma and Panic put it back to 1; each change is a fresh `0x007D` | `on_attack`, `buff::combo_hit` | `combo_attack_gains_an_orb_per_landed_swing_up_to_y_and_a_finisher_spends_them` |
| Soul Arrow | no arrow taken | `spend_attack_arrows` | `soul_arrow_stops_the_arrows_for_every_shot` |
| Bless | `x` added to Heal's rate while held | `buff::heal_cast` | `heal_restores_a_percent_of_the_ceiling_and_bless_raises_the_percent` |

The order the guards take their share in `on_user_hit` is Invincible, Power Guard, Meso Guard,
then Magic Guard - so Magic Guard's split works on what the physical guards left. `[I]`;
nothing in the client's data orders them.

**Heal** (`2301001`) restores `x`% of the drawn ceiling - *"Recovery rate 40%"* at level 1 - to
the caster and to the party on the field (`Event::PartyHeal`, computed on each recipient's own
session for the same reason a party buff crosses as a fact). Both opcodes are handled, because
no Heal has ever been captured and the client may send it as a magic attack when undead are in
range. `[I]` for reading the rate as a percent of max HP: the client's Heal formula is not
decoded, and this is the plainest reading of the number.

**Drain** (`4101002`) heals `x`% of the damage dealt, `prop`% of the time. **MP Eater** (all
three ids) absorbs `x`% of the mob's `maxMP` on a landed magic hit, `prop`% of the time, then
waits its `cooltime` of 5 s; the mob's `maxMP` is column 3 of `mobtemplates.txt`. The third-job
warriors' **Improved MP Recovery** (`1110000`, `1210000`) is a **flat** `x` per regen tick -
*"Recover 3 additional MP"* - not the Magician's percent, despite the name. `[L]` for all the
columns.

## 4. Cost only, and why that is the right answer for each

Sixteen skills are priced and produce nothing on the server, deliberately:

* **Teleport** x3, **Flash Jump** x2, **Wind Step**, **Evasion Step** - the client moves the
  character itself; the server owes the MP and now takes it.
* **The four summons and both Puppets** (`3111004`, `3211004`, `2311005`, `3111000`,
  `3211000`) - no summon packet is decoded in this client. MP is taken; the Summoning Rock is
  **not** (§1.1's rule).
* **Mystic Door** - no portal mechanics. MP taken, rocks not.
* **Meso Saver** (`4111000`) - a death save the server does not model. MP only.
* **Pickpocket** (`4211003`) - would need meso drops on every hit; **Meso Explosion**
  (`4211004`) would need them on the floor to explode. Neither is granted, on purpose: an icon
  for an effect that cannot happen is more misleading than a cost.
* **Steal** (`4201002`) is an attack (costs, damage) whose theft and success buff are not built.

## 5. The one packet that is not decoded: the mob's temporary stat

Eleven active skills and the status halves of a dozen attacks do one thing: put a state on the
**mob** - Slow, Seal, Stun, Freeze, Doom's morph, Threaten's and the three Crashes' debuffs,
Shadow Web's snare, the bleeds and burns and poisons. `research/first-job-buffs.md` §5 records
that Disorder is the same shape and that **this server has no packet for it**. Nothing here
changes that: each of those casts is priced and answered, the hit lines land, and the status is
not sent. It is one opcode to find - the mob-side sibling of `0x007D` - and it would close all
of them at once.

## 6. Passives

Eighteen are the client's own arithmetic and need nothing from the server: every mastery,
Critical Throw, Claw Guard, the four resistances, Mortal Blow's proc. Five have a server half
and it is built (§3: MP Eater x3, Improved MP Recovery x2). **Four have a server half that is
not built**, and the table says so: the Spear and Polearm Final Attacks' *"absorb 5% of damage
as HP"* (needs the Final Attack line identified inside the attack packet), Critical Recovery
(needs the per-hit critical flag wired to a heal - the flag is parsed, `net::attack::AttackHit`,
so this is close), Nimble Recovery (needs an evade signal nobody has found), and Chakra's
auto-heal at half HP.

## 7. The hidden thirteen

`invisible = 1`; `secondjob::HIDDEN_SKILLS`; never granted by `!learn` or by a skill-up. If the
client sends one as a hit - the Arrow Bomb pair works that way, `3101004` the shot and
`3101005` the blast - the costs, bullets and damage come from its row like any other. `[L]`

## 8. What a run would settle, and what it cannot

T19 in `tools/test-server.ps1` is the list. The one item that decides a convention rather than
confirms a number is **Combo Attack's orb count on cast**: zero orbs promotes the value-plus-one
reading; one orb says every Combo number is one high, and the fix is one constant. The one item
that is a regression check on old code is **Magic Guard staying on**.

What no run can settle from this side: whether the client reads the level or the `x` for the
flag bits (§2) - only a wrong icon or a wrong effect would say, and the test plan names which.

## 9. Every skill

Generated from the census by the same column rules the server uses, plus the explicit map of
which server halves exist. A row that says **not built** is a row that was looked at.

### 110 Fighter

| skill | name | what the server does |
|---|---|---|
| `1100000` | Sword Mastery | PASSIVE, client-side: mastery + W.Def |
| `1100001` | Axe Mastery | PASSIVE, client-side: mastery (the bleed proc is a mob status); mob status **not sent** (bleed) - no mob-stat packet is decoded |
| `1101000` | Final Attack: Sword | BUFF `101 FinalAttack` value=level: toggle grant |
| `1101001` | Final Attack: Axe | BUFF `101 FinalAttack` value=level: toggle grant |
| `1101002` | Sword Booster | BUFF `96 Booster` value=x: grant |
| `1101003` | Axe Booster | BUFF `96 Booster` value=x: grant |
| `1101004` | Rage | BUFF `84 PAD, 86 PDD`: grant |
| `1101005` | Rush | ATTACK: costs from the row, damage as the client sent it |
| `1101006` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row, damage as the client sent it |
| `1101007` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row, damage as the client sent it |

### 120 Page

| skill | name | what the server does |
|---|---|---|
| `1200000` | Sword Mastery | PASSIVE, client-side: mastery + W.Def |
| `1200001` | Blunt Weapon Mastery | PASSIVE, client-side: mastery + ignore def |
| `1201000` | Final Attack: Sword | BUFF `101 FinalAttack` value=level: toggle grant |
| `1201001` | Final Attack: Blunt Weapon | BUFF `101 FinalAttack` value=level: toggle grant |
| `1201002` | Sword Booster | BUFF `96 Booster` value=x: grant |
| `1201003` | Blunt Weapon Booster | BUFF `96 Booster` value=x: grant |
| `1201004` | Threaten | MOB DEBUFF, **not built**: MP taken, nothing sent - Threaten; mob status **not sent** (speed/def debuff) - no mob-stat packet is decoded |
| `1201005` | Rush | ATTACK: costs from the row, damage as the client sent it |
| `1201006` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row, damage as the client sent it |
| `1201007` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row, damage as the client sent it |

### 130 Spearman

| skill | name | what the server does |
|---|---|---|
| `1300000` | Spear Mastery | PASSIVE, client-side: mastery + crit rate |
| `1300001` | Polearm Mastery | PASSIVE, client-side: mastery + speed |
| `1301000` | Final Attack: Spear | BUFF `101 FinalAttack` value=level: toggle grant; FA: **5% HP absorb NOT built** (needs the FA line identified in the attack packet) |
| `1301001` | Final Attack: Polearm | BUFF `101 FinalAttack` value=level: toggle grant; as 1301000 |
| `1301002` | Spear Booster | BUFF `96 Booster` value=x: grant |
| `1301003` | Polearm Booster | BUFF `96 Booster` value=x: grant |
| `1301004` | Iron Will | BUFF `86 PDD`: grant |
| `1301005` | Rush | ATTACK: costs from the row, damage as the client sent it |
| `1301006` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row, damage as the client sent it |
| `1301007` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row, damage as the client sent it |

### 111 Crusader

| skill | name | what the server does |
|---|---|---|
| `1110000` | Improved MP Recovery | PASSIVE: **built**: +x MP flat per regen tick |
| `1111000` | Combo Attack | BUFF `107 ComboAttack` value=1, then per hit: toggle grant + **orbs** per landed swing, Coma/Panic spend |
| `1111001` | Panic | ATTACK: costs from the row, damage as the client sent it; spends the Combo orbs; mob status **not sent** (accuracy debuff) - no mob-stat packet is decoded |
| `1111002` | Coma | ATTACK: costs from the row (hpCon), damage as the client sent it; spends the Combo orbs; mob status **not sent** (bleed) - no mob-stat packet is decoded |
| `1111003` | Shout | ATTACK: costs from the row, damage as the client sent it; mob status **not sent** (evasion debuff) - no mob-stat packet is decoded |
| `1111004` | Armor Crash | MOB DEBUFF, **not built**: MP taken, nothing sent - Armor Crash; mob status **not sent** (defence 0) - no mob-stat packet is decoded |
| `1111005` | Power Guard | BUFF `100 PowerGuard` value=x: grant + **reflect** in on_user_hit |

### 121 White Knight

| skill | name | what the server does |
|---|---|---|
| `1210000` | Improved MP Recovery | PASSIVE: **built**: +x MP flat per regen tick |
| `1211000` | Charged Blow | ATTACK: costs from the row (hpCon), damage as the client sent it |
| `1211001` | Fire Charge | BUFF `106 WeaponElemCharge` value=level: grant |
| `1211002` | Ice Charge | BUFF `106 WeaponElemCharge` value=level: grant; mob status **not sent** (freeze) - no mob-stat packet is decoded |
| `1211003` | Lightning Charge | BUFF `106 WeaponElemCharge` value=level: grant |
| `1211004` | Elemental Crash | MOB DEBUFF, **not built**: MP taken, nothing sent - Elemental Crash; mob status **not sent** (resist strip) - no mob-stat packet is decoded |
| `1211005` | Power Guard | BUFF `100 PowerGuard` value=x: grant + **reflect** in on_user_hit |

### 131 Dragon Knight

| skill | name | what the server does |
|---|---|---|
| `1310000` | Partial Resistance | PASSIVE, client-side: resistances |
| `1311000` | Piercing Crusher | ATTACK: costs from the row, damage as the client sent it |
| `1311001` | Dragon Fury | ATTACK: costs from the row (hpCon), damage as the client sent it |
| `1311002` | Dragon Roar | ATTACK: costs from the row (hpCon), damage as the client sent it; mob status **not sent** (stun) - no mob-stat packet is decoded |
| `1311003` | Power Crash | MOB DEBUFF, **not built**: MP taken, nothing sent - Power Crash; mob status **not sent** (attack debuff) - no mob-stat packet is decoded |
| `1311004` | Dragon Blood | BUFF `105 DragonBlood + 84 PAD` value=level; indiePad: toggle grant + **HP drain** tick |
| `1311005` | Hyper Body | BUFF `94 MaxHP (party) + **server HP ceiling**`: grant |

### 210 Wizard (F/P)

| skill | name | what the server does |
|---|---|---|
| `2100000` | MP Eater | PASSIVE: **built**: prop% per landed magic hit, x% of the mob's maxMP, 5 s cooltime |
| `2101000` | Meditation | BUFF `85 MAD (party)`: grant |
| `2101001` | Teleport | COST ONLY: Teleport - the client moves itself |
| `2101002` | Slow | MOB DEBUFF, **not built**: MP taken, nothing sent - Slow; mob status **not sent** (slow) - no mob-stat packet is decoded |
| `2101003` | Fire Arrow | ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it; mob status **not sent** (burn DoT) - no mob-stat packet is decoded |
| `2101004` | Poison Breath | ATTACK: costs from the row (Poison Breath: no mad column, the hit lines are what the client sent, bullets=bulletCount on shoot), damage as the client sent it; mob status **not sent** (poison DoT) - no mob-stat packet is decoded |
| `2101005` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row, damage as the client sent it; mob status **not sent** (poison DoT) - no mob-stat packet is decoded |

### 220 Wizard (I/L)

| skill | name | what the server does |
|---|---|---|
| `2200000` | MP Eater | PASSIVE: **built**: as 2100000 |
| `2201000` | Meditation | BUFF `85 MAD (party)`: grant |
| `2201001` | Teleport | COST ONLY: as 2101001 |
| `2201002` | Slow | MOB DEBUFF, **not built**: MP taken, nothing sent - Slow; mob status **not sent** (slow) - no mob-stat packet is decoded |
| `2201003` | Cold Beam | ATTACK: costs from the row, damage as the client sent it; mob status **not sent** (slow) - no mob-stat packet is decoded |
| `2201004` | Thunder Bolt | ATTACK: costs from the row, damage as the client sent it; mob status **not sent** (shock) - no mob-stat packet is decoded |

### 230 Cleric

| skill | name | what the server does |
|---|---|---|
| `2300000` | MP Eater | PASSIVE: **built**: as 2100000 |
| `2301000` | Teleport | COST ONLY: as 2101001 |
| `2301001` | Heal | HEAL: **heals** x% of max HP to caster and party on the map (both opcodes); undead damage is the client's hit lines; MP from the row; **heals** x% of max HP to caster and party on the map (both opcodes); undead damage is the client's hit lines |
| `2301002` | Invincible | BUFF `103 Invincible` value=x: grant + **damage cut** in on_user_hit |
| `2301003` | Bless | BUFF `88 ACC, 89 EVA (party) + **Heal bonus**`: grant |
| `2301004` | Holy Arrow | ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it |

### 211 Mage (F/P)

| skill | name | what the server does |
|---|---|---|
| `2110000` | Elemental Resistance | PASSIVE, client-side: resistances |
| `2111000` | Element Amplification | BUFF `108 ElementAmp` value=level: toggle grant + **MP costs x120..150%** |
| `2111001` | Explosion | ATTACK: costs from the row, damage as the client sent it |
| `2111002` | Poison Mist | ATTACK: costs from the row (Poison Mist: the cloud is not summoned, hit lines as sent, bullets=bulletCount on shoot), damage as the client sent it; mob status **not sent** (poison mist) - no mob-stat packet is decoded |
| `2111003` | Element Composition | ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it |
| `2111004` | Seal | MOB DEBUFF, **not built**: MP taken, nothing sent - Seal; mob status **not sent** (seal) - no mob-stat packet is decoded |
| `2111005` | Spell Booster | BUFF `109 SpellBooster` value=x: grant + Magic Rock taken |
| `2111006` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row |

### 221 Mage (I/L)

| skill | name | what the server does |
|---|---|---|
| `2210000` | Elemental Resistance | PASSIVE, client-side: resistances |
| `2211000` | Element Amplification | BUFF `108 ElementAmp` value=level: toggle grant + **MP costs x120..150%** |
| `2211001` | Ice Strike | ATTACK: costs from the row, damage as the client sent it; mob status **not sent** (slow) - no mob-stat packet is decoded |
| `2211002` | Thunder Spear | ATTACK: costs from the row, damage as the client sent it; mob status **not sent** (shock) - no mob-stat packet is decoded |
| `2211003` | Element Composition | ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it |
| `2211004` | Seal | MOB DEBUFF, **not built**: MP taken, nothing sent - Seal; mob status **not sent** (seal) - no mob-stat packet is decoded |
| `2211005` | Spell Booster | BUFF `109 SpellBooster` value=x: grant + Magic Rock taken |

### 231 Priest

| skill | name | what the server does |
|---|---|---|
| `2310000` | Elemental Resistance | PASSIVE, client-side: resistances |
| `2311000` | Dispel | MOB DEBUFF, **not built**: MP taken, nothing sent - Dispel (no player debuff exists to cure, so a no-op either way); mob status **not sent** (cure (no player debuffs exist)) - no mob-stat packet is decoded |
| `2311001` | Mystic Door | COST ONLY: **no door**: MP taken, rocks NOT taken (nothing produced) |
| `2311002` | Holy Symbol | BUFF `110 HolySymbol` value=x: grant + **EXP bonus** in exp_for_kill |
| `2311003` | Shining Ray | ATTACK: costs from the row, damage as the client sent it |
| `2311004` | Doom | MOB DEBUFF, **not built**: MP taken, nothing sent - Doom; mob status **not sent** (morph) - no mob-stat packet is decoded |
| `2311005` | Summon Dragon | COST ONLY: **no summon**: MP only |

### 310 Hunter

| skill | name | what the server does |
|---|---|---|
| `3100000` | Bow Mastery | PASSIVE, client-side: mastery + evasion |
| `3100001` | Amazon's Judgement | PASSIVE, client-side: crit proc (the slow it applies is a mob status: not sent); mob status **not sent** (slow on crit) - no mob-stat packet is decoded |
| `3101000` | Final Attack: Bow | BUFF `101 FinalAttack` value=level: toggle grant |
| `3101001` | Bow Booster | BUFF `96 Booster` value=x: grant |
| `3101003` | Soul Arrow: Bow | BUFF `104 SoulArrow` value=level: grant + **no arrows taken** |
| `3101004` | Arrow Bomb: Bow | ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it; mob status **not sent** (stun) - no mob-stat packet is decoded |
| `3101005` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row (bullets=none), damage as the client sent it; mob status **not sent** (stun) - no mob-stat packet is decoded |
| `3101006` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it |

### 320 Crossbowman

| skill | name | what the server does |
|---|---|---|
| `3200000` | Crossbow Mastery | PASSIVE, client-side: mastery + ignore def |
| `3200001` | Amazon's Judgement | PASSIVE, client-side: crit proc (the slow it applies is a mob status: not sent); mob status **not sent** (slow on crit) - no mob-stat packet is decoded |
| `3201000` | Final Attack: Crossbow | BUFF `101 FinalAttack` value=level: toggle grant |
| `3201001` | Crossbow Booster | BUFF `96 Booster` value=x: grant |
| `3201003` | Soul Arrow: Crossbow | BUFF `104 SoulArrow` value=level: grant + **no arrows taken** |
| `3201004` | Iron Arrow: Crossbow | ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it |
| `3201005` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it |

### 311 Ranger

| skill | name | what the server does |
|---|---|---|
| `3110000` | Mortal Blow | ATTACK: costs from the row (bullets=bulletConsume), damage as the client sent it |
| `3111000` | Puppet | COST ONLY: **no puppet**: MP only |
| `3111001` | Inferno | ATTACK: costs from the row (bullets=bulletConsume), damage as the client sent it; mob status **not sent** (burn DoT) - no mob-stat packet is decoded |
| `3111002` | Arrow Rain | ATTACK: costs from the row (bullets=bulletConsume), damage as the client sent it |
| `3111003` | Strafe | ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it |
| `3111004` | Silver Hawk | COST ONLY: **no summon**: MP only |
| `3111005` | Wind Step | COST ONLY: Wind Step - client movement |
| `3111006` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row (bullets=bulletConsume), damage as the client sent it; mob status **not sent** (burn DoT) - no mob-stat packet is decoded |

### 321 Sniper

| skill | name | what the server does |
|---|---|---|
| `3210000` | Mortal Blow | ATTACK: costs from the row (bullets=bulletConsume), damage as the client sent it |
| `3211000` | Puppet | COST ONLY: **no puppet**: MP only |
| `3211001` | Blizzard | ATTACK: costs from the row (bullets=bulletConsume), damage as the client sent it; mob status **not sent** (slow) - no mob-stat packet is decoded |
| `3211002` | Arrow Eruption | ATTACK: costs from the row (bullets=bulletConsume), damage as the client sent it |
| `3211003` | Strafe | ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it |
| `3211004` | Golden Eagle | COST ONLY: **no summon**: MP only |
| `3211005` | Evasion Step | COST ONLY: as 3111005 |
| `3211006` | *(hidden)* | hidden `invisible=1`: never granted; if the client sends it as a hit, costs/bullets/damage apply from its row; ATTACK: costs from the row (bullets=none), damage as the client sent it; mob status **not sent** (slow) - no mob-stat packet is decoded |

### 410 Assassin

| skill | name | what the server does |
|---|---|---|
| `4100000` | Claw Mastery | PASSIVE, client-side: mastery + attack (star retrieve NOT built) |
| `4100001` | Critical Throw | PASSIVE, client-side: crit rate/damage |
| `4100002` | Critical Recovery | PASSIVE: **NOT built**: needs the crit flag per hit wired to a heal |
| `4101000` | Claw Booster | BUFF `96 Booster` value=x: grant |
| `4101001` | Haste | BUFF `92 Speed, 93 Jump (party)`: grant |
| `4101002` | Drain | ATTACK: costs from the row (bullets=bulletCount on shoot), damage as the client sent it; **heals** prop% of the time, x% of damage dealt |

### 420 Bandit

| skill | name | what the server does |
|---|---|---|
| `4200000` | Dagger Mastery | PASSIVE, client-side: mastery + crit |
| `4200001` | Nimble Recovery | PASSIVE: **NOT built**: needs an evade signal |
| `4201000` | Dagger Booster | BUFF `96 Booster` value=x: grant |
| `4201001` | Haste | BUFF `92 Speed, 93 Jump (party)`: grant |
| `4201002` | Steal | ATTACK: costs from the row, damage as the client sent it; mob status **not sent** (steal + self buff) - no mob-stat packet is decoded |
| `4201003` | Savage Blow | ATTACK: costs from the row, damage as the client sent it |

### 411 Hermit

| skill | name | what the server does |
|---|---|---|
| `4110000` | Claw Guard | PASSIVE, client-side: W.Def + block |
| `4111000` | Meso Saver | COST ONLY: **no death save**: MP/mesos not modelled; MP only |
| `4111001` | Shadow Partner | BUFF `113 ShadowPartner` value=level: grant + Summoning Rock taken |
| `4111002` | Shadow Web | MOB DEBUFF, **not built**: MP taken, nothing sent - Shadow Web; mob status **not sent** (snare) - no mob-stat packet is decoded |
| `4111003` | Shadow Meso | PASSIVE: **mesos taken** per throw (moneyCon) |
| `4111004` | Avenger | ATTACK: costs from the row (bullets=bulletConsume), damage as the client sent it |
| `4111005` | Flash Jump | COST ONLY: Flash Jump - client movement |

### 421 Chief Bandit

| skill | name | what the server does |
|---|---|---|
| `4210000` | Chakra | PASSIVE: **NOT built**: auto-heal at 50% HP |
| `4211000` | Meso Guard | BUFF `112 MesoGuard` value=x: toggle grant + **mesos pay for damage** in on_user_hit |
| `4211001` | Assaulter | ATTACK: costs from the row, damage as the client sent it; mob status **not sent** (stun) - no mob-stat packet is decoded |
| `4211002` | Band of Thieves | ATTACK: costs from the row, damage as the client sent it |
| `4211003` | Pickpocket | COST ONLY: **no meso drops**: MP only |
| `4211004` | Meso Explosion | COST ONLY: **no meso drops to explode**: MP only; any hit lines land as sent |
| `4211005` | Flash Jump | COST ONLY: as 4111005 |
