# Thief and Warrior audit, 2026-09-06

The owner: *"figure out Thief and Warrior skills. Particularly that Thief skills/basic attack should
consume stars similar to bowman with arrows, and they should be able to recharge stars at
general merchants. Also party buffs should apply to everyone in the party who is in the same
map."*

Four things came out of it, one of them a finding about an instrument rather than a feature.
Nothing here has been on a screen; the test plan (`tools/test-server.ps1`, T16 / 0f-0h) says
what each outcome would mean.

## 1. Stars are taken per throw, and Lucky Seven takes two

`Session::spend_attack_arrows` (`crates/world/src/session/combat.rs`) handled a Bow and a
Crossbow and returned early for everything else. A Claw (`147xxxx`) now draws from the whole
`207xxxx` family - the same two-range test the client's own bundle decoder makes at
`0x1403044ed` (`net::bag::BUNDLE_SERIAL_RANGES`), so Subi through Hwabi and the three event
stars all count.

| attack | held | cost | standing |
|---|---|---|---|
| plain throw | claw | 1 | plain-shot rule, same as a bow |
| Lucky Seven `4001003` | claw | **2** | `bulletCount 2`, no `bulletConsume` column; charged one per projectile - the owner's "depending on the attack amount" **[I]** |
| Double Stab `4001002` | dagger | 0 | a dagger is not a throwing weapon; never reaches the range match |

The Lucky Seven arm is the one change to the archer's rule: `BulletDuty::ProjectilesNoConsumeColumn(fired)`
used to charge the plain-shot 1 and now charges `fired` when the opcode is a shot. It moves
nothing for the Archer - Double Shot has the column, Power Knockback fires nothing - and
nothing for wands, which return before the match.

**Which stack.** The forty-field attack header (`net::attack::AttackHeader`) has no inventory
slot in it, so the lowest matching stack drains first. A Rogue carrying two kinds of star will
see the lower slot go down whichever kind the client's star icon shows; the `0x0070` keeps the
two counts honest.

### The instrument caveat, which decides whether any of this runs

**No `0x00E0` shoot body has ever been captured.** 67 archived logs carry a `0x00DF` melee;
none carries a shot (grep over `previous-runs/` and `research/fixtures/`, 2026-09-06). The
parser both the arrows and the stars go through was decoded from melee bodies. All three attack
opcodes share one encoder, `FUN_140f31fe0` (`net::attack` module docs), which is why one parser
serves them - but a shot that failed to parse would take nothing and say nothing. It now logs
`a 0x00E0 SHOOT body did not parse` instead of returning quietly. **If a throw or a shot takes
nothing on the client, look for that line first.** That would be the finding, and it would
apply to yesterday's archer work too.

## 2. Stars recharge at the general stores

The classic counter's row for a `207xxxx`/`233xxxx` id carries eight extra bytes at `row+0x40`,
an IEEE double, and `FUN_141fb9240` refuses to recharge while it reads `0.0`
(`research/classic-shop-rows.md` §3 row 41a). This server had been writing `0` there since the
day the width bug was fixed, which kept Recharge unreachable on purpose.

* **The price is measured**, out of `Item/Consume/0207.img` with `wz-dump`, and now a sixth
  column of `gm-handbook/itemdata.txt` (`tools/dump_itemdata.py`, `unitPrice`):

  | star | `unitPrice` | `slotMax` |
  |---|---|---|
  | Subi 2070000 | 0.3 | 500 |
  | Wolbi 2070001 | 0.4 | 500 |
  | Mokbi 2070002 | 0.5 | 700 |
  | Kumbi 2070003 | 0.6 | 700 |
  | Tobi 2070004 | 0.7 | 1000 |
  | Steely 2070005 | 0.8 | 1000 |
  | Ilbi 2070006 | 0.9 | 800 |
  | Hwabi 2070007 | 1.0 | 800 |
  | Snowball, Wooden Top, Icicle | 1.0 | 800 |

  Kept as thousandths (`ItemData::unit_price_milli`) so the structs stay `Eq`; converted to
  the double at the one place it is written (`ClassicShopRow::write`). The first five columns
  of the regenerated file are byte-identical to the old one; every reader accepts five or six.

* **Which counters.** The client offers Recharge for an id the open counter lists with a
  non-zero double, and every Grocer in `data/shops.txt` already lists Subi at 500 - Lucy, Mina,
  Luna, Len, Arturo, Dr. Faymus, Valen, Luma, Edel, Hana. So this is one field on rows that
  already went out, not a new row: `open_shop_for` sets the unit price on the Buy row **and its
  Sell twin**, so whichever row the client's Recharge list resolves the id to, it reads a
  price. A star a counter does not list (Wolbi at Lucy) is refused; a shop that does not stock
  a star should not recharge it, and that is a `shops.txt` edit if the owner wants it otherwise.

* **What it costs.** `ceil((slotMax - held) * unitPrice)` whole mesos, and the top-up is a
  `buy_item` of exactly that many units at that total in one transaction, so it fills the slot
  the player pointed at (the merge fills partial stacks first) and cannot half-happen. The
  rounding direction is **[I]**: the client formats its own *"Recharge: %lld"* (string `0x4B0`)
  and the arithmetic behind that string was not read - the row renderer at `0x141faf1e2` loads
  the double and compares it with zero, then builds the button; the price is computed
  elsewhere. The plan asks for the number the window shows against the number the mesos move
  by; a difference is the rounding rule and it is a one-line change.

* **Every path answers.** The window latches on send, so a potion, an unlisted star, a full
  stack, an empty slot and an unaffordable top-up are each a `0x055E`, and none moves a meso.

## 3. Slash Blast costs HP, and had not

`Obligation::deduct_hp` and `CastNumbers::hp_con` had described Slash Blast's 3..8 HP since
2026-08-28. `on_attack` read the MP and never the HP - the "built, not wired" shape, the second
one this week in the same function. `spend_attack_costs` (was `spend_attack_mp`) now takes both,
in one `0x007C`. HP floors at **1**: a skill's own cost never kills its caster. The client does
not let a 3-HP swing go out at 1 HP anyway, so the floor repairs a desync rather than granting
anything. Power Strike has no `hpCon` and is unchanged. The rest of the Warrior's book -
Iron Body's grant, the weapon classes, multi-target Slash Blast - was already wired.

## 4. Party buffs reach the party on the same map

There is no first-job party buff, so this is second-job work: **Haste** (`4101001` Assassin,
`4201001` Bandit) and **Rage** (`1101004` Fighter).

### The bits

| column | CTS bit | standing |
|---|---|---|
| `indieSpeed` | 92 | [L] - Nimble Feet, on a client |
| `indieJump` | 93 | **[D]** - name table `93 -> Jump`; decoder block identical to 92's |
| `indiePad` | 84 | **[D]** - name table `84 -> PAD`; decoder block identical to 92's |
| `indieMad` | 85 | [D] - name table; unused today |
| `indiePdd` | 86 | Iron Body's bit, drawn on a client |
| `indieMdd` | 87 | Magic Armor's second bit |

"Decoder block identical" is measured, not assumed: `research/msexe-secondarystat-140a165f0.txt`'s
census gives bits 84, 85, 92 and 93 the same 87-instruction block, `reads=u32,u16,u32,u32`,
so a Haste or Rage packet is byte-for-byte the shape the client has already accepted for Speed
and for Magic Armor's pair. What is **not** established for 84 and 93 is a reader that
consumes the slot - the same blind spot `CTS_AVOIDABILITY` records. `research/magic-damage.md`
§7.3 names `totals+0x00` the physical multiplier under the OLDER 323-name table's "83 = PAD",
which the 408-name table disagrees with by one; that row corroborates nothing either way. The
plan says which screen outcome promotes each bit and which one says it is off by one.

### What makes a buff a party buff

The archive has no column that says "party". What Rage and both Hastes carry and the self
buffs lack is the `lt`/`rb` rectangle (`ltX -250 .. rbX 250`, widening with level) with
`processtype 17`; Iron Body, Focus, Magic Armor and **Iron Will** have no rectangle and
`processtype 6`. `SkillCombat::party_rect` is that test, and it is **[I]** that the rectangle is
the discriminator - every reference server reads it so, and it is one row to change.

**Iron Will is self-only in this client's data.** Its tooltip says only "Weapon Def. +N; N%
chance to ignore knockbacks" - no party wording - and the name table has a separate flag bit,
`98 IronWill`, presumably for the knockback chance, which is not sent (its block shape was
not checked). Reported here so a run does not file it as a bug.

### The plumbing

`Event::PartyBuff { skill_id, level, caster }` crosses the bus as a **fact**, like an EXP share:
each recipient builds its own `0x007D` from the same three tables the keypress uses and records
its own expiry, so the mandatory `0x007E` comes from the session that owns that client. The
caster's `on_skill_use` fans out after its own grant, to the party roster filtered by
`Bus::characters_on(map, ...)` - "same map" is the owner's rule and is exactly what is checked; the
rectangle's size is not. No MP is spent and no cooldown stamped on a recipient.

The buff levels themselves come from the generated table (`Session::table_buff_level`), read
straight off the `indie*` columns when the row has a `time` and one or two mapped grants; three
or more is refused rather than halved, so a Bless-shaped skill lands on the "does not grant"
notice instead of on half a buff. `!buff` now consults the same three tables, so a GM on a
first-job character can put Haste on themselves to look at bits 92/93 without a second job.

## What a run decides

* A throw takes 1, Lucky Seven 2 - or `SHOOT body did not parse` appears, which is the bigger
  finding (§1).
* Recharge fills the stack; the window's number against the meso delta decides ceil vs
  truncate (§2).
* Slash Blast's HP bar ticks 3 per swing (§3).
* Haste on the recipient: faster walk **and** higher jump promotes 93; faster walk alone says
  the pair is off. Rage: the attack line moves promotes 84; icon alone says try 83 (§4).
