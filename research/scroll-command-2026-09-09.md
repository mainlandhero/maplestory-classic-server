# `!scroll` - three scrolls this client does not have, 2026-09-09

**Specification only. Nothing is implemented and nothing should be until the owner says so.**

The owner's spec, recorded here so a long session cannot lose it. Where a sentence of their is the
requirement, it is quoted rather than paraphrased.

## The command

`!scroll` behaves like `!tool`: it opens **Maple Administrator** NPC dialogue and implements
functionality the game does not have. The owner, on `!tool` originally: *"Just use MapleStory
administrator as the NPC icon."* The dialogue must

* **ask the player to confirm their choice**, and
* **let them pick which currently-equipped item to use the scroll on**.

*"Upon usage, the item is deducted from the player's inventory and the scroll action
performed."* So the scroll item is consumed on **every** use, success or failure.

## The three items

They are existing items repurposed - none of the three real scrolls exists in this client.

| item | acts as | behaviour, quoted |
|---|---|---|
| **4001009** Event Trophy | Innocence Scroll 100% | *"reverts the item to its base clean state and returns all enhancement slots to the item. This scroll does not need enhancement slots available on the item to be used."* |
| **4031065** Scroll of Secrets | Chaos Scroll 60% | *"takes 1 enhancement slot from the item. Success chance will be 100% if this is the first time user has scrolled with this item today. Upon success, randomly rolls one item stat to go up or down 0 to 5 points. This will only change stat that the item already has as a base stat."* |
| **4031066** Treasure Scroll | Clean Slate 60% | *"returns 1 enhancement slot to the item of a previously failed scroll slot. Success chance will be 100% if this is the first time user has scrolled with this item today."* |

### Clean Slate, clarified 2026-09-09 - it is a COUNTER, not a last-action flag

The owner: *"The Clean Slate should work for any previously failed scroll on the item. If the item
previously had 2 failed scroll slots, the player is allowed to use 2 clean slate scrolls on the
item. It doesn't necessarily need to be immediately after the failed scroll."*

So the per-equip state is **how many slots this item has lost to failed scrolls**, decremented
by each successful Clean Slate. Their earlier sentence - *"This will not return an enhancement
slot if the previous scroll action was successful"* - is then simply the case where that
counter is zero, not a separate rule about ordering.

## Drops

All three into the **global** drop table at **0.01% each**. That is exactly representable:
`droptables::BASIS_POINTS` is `10_000`, so 0.01% is `chance_bp: 1`, and `GLOBAL_KEY` is `"*"`.
No rounding, no unit question - which is worth stating because three bugs in this project were
a correct number in the wrong unit.

## What this can be built on, all of it already present

* **`EquipOptions::remaining_enhancements`** - `u8` at `item+0xfa`, the client's
  *"Remaining Enhancements: %d"*. This IS the enhancement-slot counter the spec turns on. The
  client compares it against `ITEMINFO.tuc`, so keep it in `0..=tuc`. **[L]**
* **`gm-handbook/equips.txt`** carries `tuc` and every base stat per item id, so "base clean
  state" and "returns all enhancement slots" are both a table lookup.
* **Equip stats are persisted per item**, 26 columns including `remaining_enhancements`
  (`store::inventory::EQUIP_STAT_COLUMNS`), so a scrolled item keeps its stats.
* **`EquipStatSet`** has all 17 stat fields, which is the set Chaos rolls within - restricted
  to those the base item has non-zero, per the spec.
* **`store::dailyperks`** already implements a once-per-UTC-day claim with both
  `SCOPE_CHARACTER` and `SCOPE_ACCOUNT`, which is the shape the 100%-first-time rule needs.

**One thing is genuinely new**: the per-equip failed-slot counter. Nothing stores it today.

## Open, and deliberately not guessed

Two questions were put to the owner and dismissed rather than answered, so they are open. Both are
economy decisions rather than technical ones, and both are player-visible and irreversible:

1. **What is the daily 100% gate counted per?** Per scroll type per character, per equip per
   character, or per scroll type per account. This decides the state model: the first reuses
   `dailyperks` almost unchanged, the second needs new per-item daily state.
2. **When Chaos fails its 40%, is the enhancement slot still consumed?** The real Chaos Scroll
   consumes it either way, and *"this scroll takes 1 enhancement slot from the item"* reads as
   unconditional - but if failure does not cost a slot, then failed slots are rare and Clean
   Slate has little to do.

A third, smaller, defaulted rather than asked: Chaos rolling a stat *down* is clamped at zero,
because a negative equip stat is not representable in `EquipStatSet` (`u16`).

## Note on ordering

Real scrolling (`0x0125`, `research/scrolling-2026-09-09.md`) is decoded but **not
implemented**. Until it is, these three are the only things that can create or consume an
enhancement slot, so the failed-slot counter has exactly one writer. That is a good state to
build in, and a reason to do `!scroll` before or alongside `0x0125` rather than after.

---

# The spec changed, later the same day

Everything above is the ORIGINAL spec and is kept as written, because two of its sentences are
still the rule and the third is not. The owner, after seeing the first build on a screen:

> *"Okay I lied, plan change. Do not use event trophy since it does not stack."*

and, as draft patch notes:

> **Scroll of Secrets (Global Drop 0.01% chance)** - Functions either one of the following
> scrolls
> * *Chaos Scroll (100% first time of the day, otherwise 60%), randomly increases or decreases
>   one of the item's base stat by up to 5 points. Reduces enhancement slot by 1.*
> * *Innocence Scroll (100%), returns the item back to its unmodified base state, no random
>   base stats will be kept. Returns all enhancement slots. Does not require an enhancement
>   slot to use.*
> * *Clean Slate Scroll (100% first time of the day, otherwise 60%), returns a failed
>   enhancement slot of a previous scroll you have used upon the item. You cannot recover an
>   enhancement slot if the original scroll succeeded.*
>
> **Treasure Scroll (Global Drop 0.01% chance)**
> * *Use this Scroll to automatically succeed the next scroll of your choosing via the GUI
>   options and apply those stat increases to the item immediately while subtracting an item
>   enhancement.*

## What changed, and what did not

| | before | after |
|---|---|---|
| items | three | **two** |
| `4001009` Event Trophy | the Innocence Scroll | **dropped entirely** |
| Innocence | its own item | a **mode** of the Scroll of Secrets |
| `4031065` Scroll of Secrets | Chaos only | **all three modes**, chosen in the dialogue |
| `4031066` Treasure Scroll | Clean Slate | **guarantees a real scroll** the player carries |

Unchanged: the daily gate is still per scroll type per character (now keyed on the *mode*), a
failed Chaos still eats the slot, Clean Slate is still a counter, and the repurposed item is
still consumed on every use.

## Why Event Trophy went, measured

`gm-handbook/itemdata.txt`, column 5 is `info/slotMax`:

```text
4001009, 5000, 0, 0, 0, 0     <- slotMax 0
4031065, 1, 1, 0, 1, 0        <- slotMax 1
4031066, 1, 1, 0, 1, 0        <- slotMax 1
```

**All three are 0 or 1, so none of them stacks on the client's own numbers.** The owner found it on
Event Trophy first. `crate::shops::max_stack` overrides all of them server-side to 100, and
whether that override is honoured by a client whose `slotMax` says otherwise is **[I]** - it
did not save `4001009`, and nothing has been measured about `slotMax = 1` behaving differently
from `slotMax = 0`. Asked which way to go, the owner chose **keep `4031065`/`4031066` and test on a
screen**. If the answer is that it does not stack either, 161 of the 359 Etc items already
carry `slotMax = 200`.

## The Treasure Scroll's menu source, asked and answered

*"the next scroll of your choosing via the GUI options"* has two readings and they need
opposite implementations: any real scroll that fits the item, or only one the player owns.
Asked directly, the owner chose **only a real scroll you are carrying**. It is consumed along with
the Treasure Scroll, it is guaranteed to succeed, and one enhancement slot is spent.

### Which real scrolls fit which equip - derived, and controlled

There is no applicability field in the WZ. `0204.img`'s `itemID1/2/3` are the *other tiers of
the same scroll* (`2040000` names `2040001/2/3`), not a list of targets. The rule is the id:

```text
    equip category = equip_id / 10000
    scroll category = 100 + (scroll_id % 10000) / 100
```

**Controlled against the client's own names**, which can disagree: grouping all 208 scrolls by
the derived category gives 24 groups, and every group's name begins with the category word -
*Hat*, *Earring*, *Topwear*, *Overall Armor*, *Bottomwear*, *Shoes*, *Gloves*, *Shield*,
*Cape*, *One-Handed Sword* … *Claw*, *Pet Equip*. 24 of 24 agree.

And against a screen: the owner's **1322999** Wizet Secret Agent Suitcase has a tooltip reading
*One-Handed Blunt Weapon*. `1322999 / 10000 = 132`, and `2043200`'s name is *One-Handed Blunt
Weapon Attack Scroll*. `crate::config::ScrollTemplate::category` carries both controls as a
test over the real file.

## Two bugs the first build shipped, both the same pair of stats crossed

The owner, scrolling the suitcase: *"It lost 1 weapon attack on the item, but it also gave it 200
attack power."*

* `session/scroll.rs`'s `equip_base` wrote `inc_pad: t.inc_wat`. This client's
  `Character.wz` has `incWAT` on 202 equips and **`incPAD` on none**, so bit 8 must be zero.
  The Chaos roll was correct; the *base* it was measured against was not.
* `scrolls.rs`'s `STATS` table labelled `inc_pad` **"Weapon Attack"** and `inc_wat` **"WAT"** -
  the same two crossed, in the strings read out to the player.

Both now go through the one function that owns the mapping, and the labels are the client's own
tooltip strings (`0x0380` *Attack Power*, `0x037F` *Weapon Attack*).
