# Archer audit, 2026-09-06: arrows were never taken, and the knockback is the client's

The owner: *"do an audit of the archer skills. Make sure that regular attacks or skills using
bows/crossbows should consume arrows from the use tab depending on the attack amount. for
example double shot should consume 2 arrows, and power knockback should knock back the mob
considerably compared to a normal attack."*

No client run. Everything here is `crates/`, `gm-handbook/skills.txt` (generated from this
client's own `Skill.wz`), `gm-handbook/items.txt`, and two existing research files. Tags as
everywhere: **[L]** read off this client's data or listing, **[D]** derived, **[I]** inferred.

---

## 1. Arrows: the rule existed and nothing called it

`crates/world/src/firstjob.rs` has carried `BulletDuty` since 2026-08-28, read off the skill
table's `bulletConsume` column:

| skill | id | `bulletConsume` | `bulletCount` | weapon gate | what the server owed |
|---|---|---|---|---|---|
| Arrow Blow | 3001001 | **1** | 1 | bow or crossbow | MP, **1 arrow**, damage |
| Double Shot | 3001002 | **2** | 2 | bow or crossbow | MP, **2 arrows**, damage |
| Power Knockback | 3001003 | *(none)* | *(none)* | bow or crossbow | MP, **no arrow**, damage |

All **[L]**, constant across every level row. `Session::on_attack` spent the MP
(`spend_attack_mp`, since 08-28) and never read `bullets`. **Built, not wired** - the shape
`CLAUDE.md` has a section about.

### 1.1 What is wired now: `Session::spend_attack_arrows`

Called from `on_attack` right after the MP, with the same never-refuse discipline:

1. **Weapon in hand decides everything.** The worn weapon is found by item class among
   `equipped_items` (`WeaponClass::from_item_id`; only weapon ids classify, so a cash cover
   in the cash slot cannot be mistaken for one). Not a bow or crossbow: no arrow, whatever the
   packet says.
2. **The cost:**
   * a skill with `bulletConsume` costs exactly that (`BulletDuty::Consume(n)`);
   * a skill with no bullet column costs nothing - Power Knockback is the bow swung as a club;
   * a skill the data does not settle (`bulletCount` with no `bulletConsume`, or a skill
     outside the first-job book) costs **one on `0x00E0 USER_SHOOT_ATTACK` and nothing on
     melee**, logged as **[I]** each time;
   * a plain attack costs one on the shoot opcode and nothing on the melee opcode. The client
     sends a bow's normal attack as a shoot **[L]**; "one arrow" is the game's rule, stated as
     such.
3. **The stack:** bows drain `2060000..=2060999`, crossbows `2061000..=2061999` -
   `gm-handbook/items.txt` names them *"Arrows for Bows"* and *"Arrows for Crossbows"*
   **[L]**. The lowest matching Use-tab slot goes first; a stack short of the cost gives what
   it has and the next stack pays the rest.
4. **The client is told** with the same two `0x0070` shapes a potion uses: mode 1 (the stack
   is now `n`) or mode 3 (the slot is empty). `consume.rs::stack_change_replies`, now shared.
5. **Never refused.** No matching stack, or not enough, is a log line and a smaller
   deduction. The client will not fire without arrows, so a shortfall is the two ends
   disagreeing about a count, and the `0x0070` for what *was* taken repairs it.

### 1.2 The attack packet carries no bullet slot

`net::attack::AttackHeader` reads all 33 header fields; most are constant across the 434
distinct captured bodies and none moves with an inventory slot. The classic client's
`nBulletItemPos` has no counterpart here that the corpus can show. So **the server chooses the
stack** - a decision, written down as one. If a later capture shows a slot field, the
lowest-slot rule becomes "the slot the client named".

### 1.3 Tests

Six, in `session/tests.rs`, all over the **real captured swing** with the skill id and level
patched into body offsets 2 and 6 (the same body `an_attack_skill_costs_mp...` uses):

* a plain shot with a bow: 50 → 49, and a mode-1 `0x0070`;
* Double Shot 50 → 48, then Arrow Blow → 47;
* Power Knockback, and a plain melee swing while holding a bow: 50 → 50;
* a crossbow ignores `2060xxx` and takes `2061xxx`;
* the last arrow empties the slot with mode 3; Double Shot with one arrow takes the one and
  is not refused; with none, the swing still goes through and no `0x0070` is sent;
* a sword never costs an arrow however the packet is labelled.

## 2. Power Knockback: what the server owes, and what it cannot do

`research/mob-hit-reaction.md` §0 settled where a mob's hit reaction comes from: **the client
that both swings and holds the mob's `0x03D2` grant produces it locally**, and it reaches the
other screens as that client's own `0x02FF` report with action 7. No packet the server sends
produces a flinch or a knockback, and there is no `MobDamaged` opcode in this client. **[L]**,
two archived runs, ten for ten in both directions.

So the knockback *distance* is the client's. For Power Knockback the number is in the skill's
own row: the `range` column, **130 at level 1 rising to 150 at level 15**, and the tooltip
column reads it back as *"knockback 2 enemies by 130"* ... *"knockback 4 enemies by 150"*
(`mobCount` 2 → 4). **[L]** A normal hit's shove is the mob's ordinary `hit1` displacement.
What "considerably" means in pixels is therefore the client's 130-150 against that, and the
server has no field to turn.

What the server owes, and its state:

| duty | state |
|---|---|
| spend `mpCon` (12 at level 1, descending to 8) | wired, `spend_attack_mp` |
| take no arrow | wired, this audit (`BulletDuty::None`) |
| apply the damage | the client's per-hit damage is applied as sent; `damage` 105..180 percent per the row |
| **hand the mob to the attacker**, because the reaction only plays for its controller | wired since 09-04 (`hand_over_one` in `on_attack`, release first then grant) |
| not fight the pushed position | the `0x03D9` relay forwards the controller's own bytes; no server-side distance check exists (grepped: none) |

Two consequences worth stating before a run:

* On a mob the **other** client was driving, the **first** Power Knockback hands the mob over
  and plays no reaction on the attacker's screen; the **second** pushes. That is the same
  first-hit rule every attack has here, not something specific to this skill.
* `time 1500` is constant at all 15 levels and its unit is unresolved (the file header says
  seconds, which is absurd for a knockback; milliseconds would be a 1.5 s stun). The server
  does not use it.

## 3. The rest of the first-job book

| skill | kind | server |
|---|---|---|
| Critical Shot 3000000 | passive | client-side: the critical flag arrives **inbound** on the attack packet, so there is no server roll to modify |
| The Eye of Amazon 3000001 | passive | client-side: `range` +50..120 pixels, the client picks its own targets |
| Focus 3001000 | buff | **acknowledged, NOT granted.** `all_granted_by(3001000)` finds no stat because neither Accuracy nor Avoidability has a known CTS bit (`net::buff`); the cancel path names 88/89 as beliefs. Same gap as Iron Body. Needs the two bits measured |

## 4. Not covered, and said so

Second-job archer skills have **no cast handlers** (`STATUS.md`: none of the 66 second-job
skills does). The data is read and the three cases that would matter for arrows are known:

* **Arrow Bomb 3101005** carries `noBulletConsume 1` - the only three skills in the archive
  with that column are 3101005, 3111006 and 3211006 **[L]**;
* **Soul Arrow** makes every shot free while it is up;
* **Strafe** and **Holy Arrow** carry `bulletCount 3` and no `bulletConsume`
  (`BulletDuty::ProjectilesNoConsumeColumn`), so the rule above would charge them one per cast
  on the shoot opcode and say [I] in the log.

None of that runs until those casts are handled. `spend_attack_arrows` will not need changing
for `noBulletConsume`; it will need to read the column when the second-job table lands.

## 5. What the owner should see, and what each outcome means

| do | see | means |
|---|---|---|
| normal shot with a bow, arrows in the Use tab | the stack drops by 1 | wired |
| Double Shot | drops by 2 | `bulletConsume` read correctly |
| Power Knockback | no change | the "no bullet column" branch |
| the same with a crossbow and `2061xxx` | drops | the class-to-range rule |
| Power Knockback on a mob **you** control | pushed ~130 px on your screen; the other screen shows the same landing spot via `0x03D9` | client distance, server relay |
| Power Knockback on a mob the **other** client controls | first hit: no push; second: pushed | the control handover, working as designed |
| a stack that does not move | read `world.log` for `arrows:` lines - either the weapon did not classify, or the stack was not in the range | the finding |
