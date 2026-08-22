# `returnMap`, `forcedReturn`, `town` — where a dead character comes back

2026-08-21. Data: `tools/dump_returnmaps.py` -> `gm-handbook/returnmaps.txt`.
Source: the client's own `Map.wz` field images, plus `Item.wz/Consume/0203.img` and
`String.wz/Map.img`. **No Ghidra, no client run.**

Labels: **[L]** read straight out of the client's data. **[D]** derived from the data by an
argument written down here. **[I]** inference with no measurement behind it.

---

## 0. The instrument, before any of its answers

426 field images, all read, all carry an `info` node. [L] That matches
`gm-handbook/fields.txt`, which is the project's existing count of maps with a field image,
so the pass is the whole map set and not a sample. [L]

`dump_returnmaps.py` refuses to write a file unless four positive controls come back
non-empty and one negative control comes back empty:

| control | result |
|---|---|
| `version` | 426/426 |
| `bgm` | 426/426 |
| `mapMark` | 426/426 |
| `fieldType` | **379**/426 |
| `returnMapZZ` (negative) | 0/426 |

`fieldType` is the useful one: it is present on some maps and genuinely absent on 47, so a
reader that returned a constant would fail it. [L]

**`VRLimit` is not a key in this client.** It was suggested as a positive control and it is
0 of 426 — the viewport bound here is four separate keys, `VRTop` / `VRBottom` / `VRLeft` /
`VRRight`, present on 413 each. [L] A control that can only ever fail is exactly the
instrument this project keeps warning about, so it is not used.

A second, independent instrument: a byte scan of `client-patched\MapleStory.exe` (76.7 MB)
for each key name in ASCII and UTF-16LE. It discriminates — `returnMapZZ` and
`notAKeyAtAll` return 0, `fieldType` / `foothold` / `portal` return hits. [L]

| key | ascii | utf16 |
|---|---|---|
| `returnMap` | 1 | 3 |
| `forcedReturn` | 0 | 1 |
| `reviveCurField` | 1 | 0 |
| `genReturnMap` | 1 | 0 |
| `moveTo` | 0 | 3 |
| `town` | 9 | 5 |
| **`mobRate`** | **0** | **0** |

So the client itself parses `returnMap`, `forcedReturn` and `moveTo`. [D] And `mobRate` —
a key on all 426 maps — is **not** in the client image at all, which corroborates
`crates/world/src/config.rs`'s standing claim that spawn rate is server policy. [D]
(Blind spot, stated so it can be quoted: a name assembled at runtime, or stored compressed,
would not show up. This scan can prove presence, never absence.)

---

## 1. The sentinel is `999999999`, and it is the same one the portal table uses

`999999999` — nine nines — is the client's "there is no literal map id here" value. It shows
up in three unrelated places and means the same thing in all of them:

* `portal/<n>/tm` — "not a door, this is a spawn point". Already relied on by
  `tools/dump_portals.py` as `NO_TARGET`. [L]
* `info/forcedReturn` — "this field ejects nobody". 354 of 426 fields. [L]
* `Item.wz` `02030000/spec/moveTo` — and that item is **`Return Scroll - Nearest Town`**
  (`gm-handbook/items.txt`). [L]

That last one is the measurement that settles what `returnMap` is *for*. Every other Return
Scroll carries a literal town id in the same field: [L]

```
02030000  moveTo 999999999   Return Scroll - Nearest Town
02030001  moveTo  10000000   Return Scroll to Lith Harbor
02030004  moveTo  10001000   Return Scroll to Henesys
02030009  moveTo  20001000   Return Scroll to El Nath
```

So the client has a notion of **"the nearest town", resolved per-field at use time**, and
`999999999` is how an item asks for it. The only per-field key that can answer is
`info/returnMap` — it is the one map id defined on every single field. [D]

**What the server must do when it sees `999999999`: never warp to it.** It is not a map id.
For `forcedReturn` it means "no rule"; for a move-to field it means "look the destination up
from the field the character is standing on".

---

## 2. `returnMap` — the field's nearest-town anchor

### What is measured [L]

* Present on **426 of 426** fields. Never missing.
* **Never** the sentinel. Value range 1 .. 900000002; never 0, never negative.
* Points at a field flagged `town == 1` on **388 of 426**.
* Equals the field itself on **28** fields (21 town centres, 7 event/test fields).
* Every `returnMap` value names a field that has an image. Zero dangling targets.

### It is not "where the exit portal goes" [D]

This is the part that had to be checked rather than assumed, because both readings are
plausible from the name alone.

Cross-referencing `gm-handbook/portals.txt`: 401 of the 426 fields have at least one door
portal (a portal with a real `tm`).

| | |
|---|---|
| `returnMap` **is** one of that field's own portal targets | **85** of 401 |
| `returnMap` is **not** reachable by any portal from that field | **316** of 401 |

And walking the portal graph outward, the distance from a field to its own `returnMap`: [L]

```
0 hops  28      (returnMap == self)
1 hop   76
2 hops  54
3 hops  43
4 hops  48
5 hops  28
6 hops  19
never  130      (no portal path at all within 6 hops)
```

**130 fields cannot walk to their own `returnMap`.** `Snail Hunting Ground I` (40) has
`returnMap` 60, *Southperry* — the port town at the other end of Maple Island, several
screens away. `In a Small Forest` (1001) has `returnMap` 1010, *Amherst*. Neither is a door
out of the field; both are the island's town.

So `returnMap` is a **teleport destination**, not a map exit. [D] The exits are in
`portals.txt` and they are a different set of numbers.

### Is it "when you DIE" or "when you leave by other means"? [D]

The honest answer, with the blind spot named:

`Map.wz` never names the death event. Nothing in the WZ says the string "death". So
"`returnMap` is the death map" cannot be **[L]** from this data and is not claimed as such.

What *is* claimed, and what the owner actually asked for, is stronger and is measured:
**`returnMap` is the field's nearest-town anchor.** The Return Scroll pair in §1 proves the
game has that concept and that it is resolved from the field; `returnMap` is the only
field-level key that can supply it; and its shape — always defined, a town 388 times out of
426, defined even on shop interiors where nothing can kill you, and unreachable on foot from
130 fields — is the shape of a fallback teleport and nothing else. [D]

Two supporting observations, both weaker, both recorded so they are not mistaken for the
argument:

* **`reviveCurField`** exists as an `info` key — on exactly one field, `89000000` *Truth
  Booth*, value 1. [L] A per-field override literally named "revive on the current field"
  only makes sense if the default revive *leaves* the field. [D] It does not discriminate
  as hard as it looks: Truth Booth's `returnMap` is already itself, so the override is
  redundant there under either reading.
* **`genReturnMap` = 1** on the six `1st Accompaniment` party-quest stages and nowhere
  else. [L] "generate the return map" — an instanced field's return is computed at run time
  rather than read. [I]

**If this is ever worth settling properly**, the cheap experiment is a client run, not more
static work: die on map 40 and see whether the client lands on 60 (Southperry) or on an
adjacent field. It is one launch and it is a claim that can come back false.

---

## 3. `forcedReturn` — eject, not respawn

### What is measured [L]

* Present on **426 of 426**.
* **354** are the sentinel `999999999` — "this field ejects nobody".
* **72** carry a real map id. Every one names a field that exists.
* Equals `returnMap` on **44** of those 72; **disagrees on 28**.
* Its target is a town on **18** of 72 — and **not** a town on **54**.
* Equals the field itself on 7 (`80000600`, `89000000`, `90000000`, `90050000`,
  `900000000`/`1`/`2`) — all of them the lobby or exit field of their own group.

### What the 72 fields are [L]

Not a random 72. Every one is a field a character must not be *left standing in*:

| group | fields | `forcedReturn` -> |
|---|---|---|
| Ellinia/Orbis ship cabins, mid-flight | `10002091`, `20000013`, `20000020..23` | the ticket booth / station |
| Kerning subway depots (these carry `timeLimit` 5940 s) | `10003102`, `10003105`, `10003109` | Kerning City |
| Henesys quest houses & Pig Park | `10001002..06`, `10001082` | Henesys |
| Sleepywood Cursed Sanctuary run | `10005100..05` | `10005080` *Another Entrance* |
| El Nath Dead Mine / Cave of Trial / Zakum door | `20001064..67`, `20001070..75`, `20001044` | the field outside |
| Job-advancement instanced dungeons | `80001000..80001300` | the field outside their door |
| `1st Accompaniment` party quest | `80000000..80000600` | `80000600` *PQ Exit* |
| Event stages | `90040000..90050001` | `90050000` *Leaving the Event* |
| Test fields | `900000000..2` | themselves |

### The 28 disagreements are the discriminator [D]

Where `returnMap` and `forcedReturn` differ, `returnMap` is the **town-ward** destination
and `forcedReturn` is the **just-outside-this-instance** one: [L]

```
An Empty House 10001002    returnMap 10001001 Henesys Townstreet   forcedReturn 10001000 Henesys
The Door to Zakum 20001075 returnMap 20001000 El Nath              forcedReturn 20001070 The Passage
Dead Mine I 20001064       returnMap 20001000 El Nath              forcedReturn 20001063 Forest of Dead Trees IV
Sanctuary Entrance I 10005101  returnMap 10005000 Sleepywood       forcedReturn 10005080 Another Entrance
```

`Dead Mine I` sends a corpse all the way home to El Nath and an ejected character only as
far as the forest outside the mine. Those are two different questions with two different
answers, and this field is the second one. [D]

The party quest inverts the direction and stays consistent: `1st Accompaniment <2nd Stage>`
has `returnMap` **80000000** (stage 1, *inside* the instance) and `forcedReturn` **80000600**
(the Exit). Dying mid-PQ puts you back at the start of the PQ; being ejected from it puts you
out of it. [D]

**So `forcedReturn` fires when a character is found on a field they may not persist on** —
login, channel change, or the field's own timer expiring — and it is **not** the revive
destination. [D] Two independent reasons it cannot be: it is absent (sentinel) on 354 fields
where a death still has to go somewhere, and where it *is* present its target is a non-town
54 times out of 72. [L]

**This server has no reason to implement `forcedReturn` for death.** It matters later, for
login placement. Record it as decoded and unwired.

---

## 4. `town`

* The key is on **426 of 426** fields. [L]
* Only two values occur: **0** (311 fields) and **1** (115 fields). No third value. [L]
* So `town == 1` marks a town field, and there are **115** of them. [L]

**Mushroom Town is among them, and so is every Maple Island starting field.** [L]

```
1     town=1  Mushroom Town - West Entrance
10    town=1  Mushroom Town
20    town=1  Mushroom Town
21    town=1  Mushroom Town Townstreet
30    town=1  East Entrance to Mushroom Town
60    town=1  Southperry
61    town=1  Southperry Armor Store
1010  town=1  Amherst
```

**The flag is coarser than "town square".** Shop and salon interiors are flagged `town == 1`
too — `Henesys Hair Salon`, `Lith Harbor Weapon Shop`, `Southperry Armor Store`. So is every
`Free Market <n>` booth, every `Forest of Patience` step, and — oddly — all thirteen
`Physical Fitness Test` rooms. [L]

This is the trap the resolver in §5 is built around: **94 of the 115 `town == 1` fields have
a `returnMap` pointing somewhere else.** Only **21** are their own `returnMap`, and those 21
are the real town centres: [L]

```
1, 10, 20, 21, 30, 60, 1010, 10000000 Lith Harbor, 10001000 Henesys,
10001001 Henesys Townstreet, 10002000 Ellinia, 10003000 Kerning City,
10004000 Perion, 10005000 Sleepywood, 10006000 Forgotten Hollow,
10007000 Florina Beach, 20000000 Orbis, 20001000 El Nath,
80002000 Free Market Entrance, 89000000 Truth Booth, 90000000 Maple Hill
```

A resolver that stops the moment it sees `town == 1` revives the player **inside the armor
shop**. One that always takes the first `returnMap` hop puts them in Southperry. [D]

---

## 5. Does every field resolve to a town in finite hops?

**Yes, with one hop of unconditional movement and a loop guard. Nothing hangs.** [L]

The rule `dump_returnmaps.py` uses for its `reviveMap` column:

1. Take **one** `returnMap` hop, unconditionally — even out of a field already flagged
   `town == 1`. (§4: 94 of 115 town fields point elsewhere.)
2. Keep hopping while the field you land on is **not** `town == 1`.
3. Stop at 8 hops, at a self-reference, or at a revisit — and **return the last real field**,
   not "nothing".

Measured over all 426: [L]

```
hops = 1   421 fields
hops = 2     5 fields
hops > 2     0 fields          <- the deepest chain in this client is two
no destination at all   0      <- every returnMap names a field that exists
lands on town == 1     393 of 426
lands on a real non-town field  33 of 426
```

The five two-hop chains, in full: [L]

```
10005078 The Grave of Mushmom       -> 10005070 Ant Tunnel Park       -> 10005000 Sleepywood
80001000 Ant Tunnel For Bowman      -> 10001090 Road to the Dungeon   -> 10001000 Henesys
80001100 Magician's Tree Dungeon    -> 10002070 Forest N of Ellinia   -> 10002000 Ellinia
80001200 Thief's Construction Site  -> 10003080 Construction Site N   -> 10003000 Kerning City
80001300 Warrior's Rocky Mountain   -> 10004023 West Rocky Mountain IV-> 10004000 Perion
```

### The 33 that never reach a town [L]

There **are** cycles, and step 3 is what stops them being a hang. All 33 fields whose walk
terminates on a non-town are party quest, event or test fields:

* `80000000..80000500` — the six `1st Accompaniment` stages, all -> `80000000` (stage 1),
  which is its own `returnMap`. **A self-loop.**
* `80000600` `1st Accompaniment <Exit>`, `80003500` (unnamed, `fieldType` 500) — own
  `returnMap`.
* `90040000..90050001` — 22 event fields, all -> `90050000` *Leaving the Event*, which is its
  own `returnMap`.
* `900000000`/`1`/`2` — `White Map`, `White Map with Mob`, `Black Map with Mob`. Own
  `returnMap`.

Terminating on the last real field is deliberate and it is the better answer: a party-quest
death lands on stage 1, which is what the data says, rather than being flung to a fallback
town. [D] The only case that produces no destination at all is a `returnMap` naming a field
this client does not have — **zero fields today** [L] — and the branch still has to exist,
because a revive with no destination freezes the client.

### None of the 33 is reachable in this server today [L]

Walking `gm-handbook/portals.txt` outward from map 1, the entire on-foot world is **25
fields**, all of Maple Island. Maple Island is closed — you leave it by NPC script, not by a
portal — so no reachable field is one of the 33, and every one of the 25 resolves in one hop:

```
1, 10, 20, 21, 30      -> themselves (town)
40, 41, 42, 50         -> 60    Southperry
60                     -> 60    Southperry
61                     -> 60    Southperry
1000..1006, 1014,
1020, 1021             -> 1010  Amherst
1010, 1011, 1012, 1013 -> 1010  Amherst
```

So the first client run of death-and-revive will exercise exactly two destinations,
Southperry and Amherst, plus the five town fields that revive in place. [D]

---

## 6. What is *not* answered here

* **Whether the client's own death path reads `returnMap`.** Not answerable from WZ, and the
  string scan only proves the name is in the image. It does not matter for the build: the
  server sends the field-change, so the server decides. But do not write "the client uses
  returnMap on death" in a doc comment.
* **Which portal a revived character arrives at.** Every one of the 426 fields has at least
  one `sp` portal [L], so `(map, "sp")` always resolves — but *which* `sp` when there are
  several (Henesys has three at indices 0/1/2) is a placement question this pass did not
  answer.
* **`reviveCurField`.** One field, `89000000`. If Truth Booth is ever reachable, the revive
  must keep the character where they are. Unwired.
* **`decHP`** (21 El Nath fields, value 10) and **`recovery`** (`10005002` Regular Sauna 2.0,
  `10005003` VIP Sauna 3.0) are the two field keys that also move HP. Out of scope here,
  noted so the next pass does not re-find them.

---

## WIRE IT LIKE THIS

Answering *"where does this character respawn"* for any map id.

**Data.** `python tools/dump_returnmaps.py` writes `gm-handbook/returnmaps.txt`, 426 rows,
**TAB separated** (one map is named `The Resting Spot, Pig Park` — a comma-separated row
splits it in the wrong place). Columns:

```
map \t returnMap \t forcedReturn \t town \t reviveMap \t hops \t name
```

`map`, `returnMap`, `forcedReturn`, `town` are read from the WZ. `reviveMap` and `hops` are
**derived by the script** with the §5 rule. `gm-handbook/` is gitignored and regenerated;
never hand-edit it.

**The constant.**

```rust
/// The client's own "there is no map id here" sentinel. Same number `portal/<n>/tm` uses
/// for a spawn point and `Return Scroll - Nearest Town` (2030000) uses for `spec/moveTo`.
/// NEVER warp to it.
pub const NO_MAP: u32 = 999_999_999;
```

**The lookup.** Load `reviveMap` into a `HashMap<u32, u32>` at startup and the answer is one
index. Do **not** re-derive the walk at revive time; do not follow `returnMap` yourself.

```rust
/// Where a character who died on `field` comes back.
///
/// `reviveMap` already encodes ONE UNCONDITIONAL `returnMap` hop followed by a walk to the
/// first `town == 1` field. The unconditional hop is not an accident: 94 of this client's
/// 115 `town == 1` fields point their `returnMap` somewhere else, because shop interiors
/// carry `town == 1` too. Stopping on the flag revives the player inside the armor shop.
///
/// The table's deepest chain is two hops and it has no cycles left in it - the generator
/// broke them. A caller that walks `returnMap` itself will find the cycles.
fn revive_field(&self, field: u32) -> u32 {
    self.revive_map.get(&field).copied().unwrap_or(DEFAULT_REVIVE_FIELD)
}
```

**The no-return case.** Two of them, and they are different:

* *`returnMap` absent, or the sentinel.* **Happens on zero of 426 fields** — `returnMap` is
  on every field and is never `999999999`. The branch still exists because the table is
  loaded from a generated file that could be truncated, and a revive that returns no field
  is an unanswered packet, which freezes the client's whole UI.
* *A field id with no row* — a map the server invented, or a field image this client does not
  ship. `unwrap_or(DEFAULT_REVIVE_FIELD)`.

`DEFAULT_REVIVE_FIELD` should be **`10001000` (Henesys)** if the server ever leaves Maple
Island, and **`60` (Southperry)** until it does. [I] It is policy, not data — say so in the
constant's doc block. It is never reached by any of the 426 rows.

**Do not use `forcedReturn` for this.** It is the sentinel on 354 fields, and where it is set
its target is a non-town 54 times out of 72. It answers "this character may not be left
standing here", which is a login-placement question. Wire it there or record it as unwired;
it is decoded either way.

**What the caller still owes, and none of it is in this file:**

* the arrival portal — use the destination's `sp` portal from `gm-handbook/portals.txt`
  (all 426 fields have one);
* HP 50 on revive, EXP −10% above level 10 — the owner's rule, server policy, nothing in the WZ
  corroborates or contradicts it;
* and per `CLAUDE.md`'s guard rule: the EXP penalty and the warp both hang off the **death
  transition**, not off the revive request. If the character was not dead, nothing may
  follow — return early on the refusal rather than gating the warp and the EXP separately.
