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
