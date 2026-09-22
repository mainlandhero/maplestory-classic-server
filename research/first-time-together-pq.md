# "First Time Together" — the Kerning party quest, as this client holds it

Research pass, 2026-09-22. Nothing here has been on a screen; **no part of this PQ is
implemented on the server today**. Every claim is tagged:

* **[L]** read out of this client's own data (`Map.wz`, `String.wz`, `Quest.wz` via
  `gm-handbook/`). These are facts about the client we ship against.
* **[I]** inferred from those facts.
* **[C]** candidate from `ModernMapleSource` (**v214, a different version** — `CLAUDE.md`
  scores it 1 of 8 against a held-out control). Useful for shape, settles nothing.

The client is internally consistent with the classic Kerning PQ, and its map data
corroborates the [C] rules in a way worth taking seriously — see *Where the two agree*.

---

## 1. The quest row

`Quest.wz` id **10311** [L]:

| | |
|---|---|
| name / parent | `Proof of Companionship` / **`First Time Together`** |
| area, order | 14, 1 |
| start check | `lvmin 21`, `npc 800000` |
| complete check | `mob 800003 × 1`, `npc 800000` |
| reward | `exp 2193`, `money 614`, **`pop 1`** (one fame) |
| reward item | exactly one of `2040301`, `2040305`, `2040309`, `2040313`, each `prop 1` |

The quest text names the entry rule in words [L]: *"a party of **four adventurers, all
level 21 or higher**… the party leader can speak to `#p800000#` to start"*. Note the
**mob check is the only completion requirement** — King Slime, once. The stages are not in
the quest row at all; they are the server's business.

`questreq.txt` agrees: `10311  1  mob  0  800003  1` [L].

## 2. The three sisters

| npc | name | where | role |
|---|---|---|---|
| `800000` | **Lakelis** | Kerning City `10003000` @ (-540, 426) [L] | the only one placed in the world; gives and takes the quest |
| `800001` | **Cloto** | in stages 1–5, never in Bonus or Exit [L] | the stage NPC |
| `800002` | **Nella** | in **all seven** maps [L] | the way out |

Lakelis is already in `gm-handbook/npcs.txt` and therefore already stands in Kerning City
on this server. Cloto and Nella exist only inside the PQ maps, which nothing can reach yet.

## 3. The maps

`80000000`–`80000600`, named **"1st Accompaniment"** [L] — the Korean name of the classic
Kerning PQ, so this is that quest and not a renamed variant.

| map | name | life | `area` rects |
|---|---|---|---|
| `80000000` | `<1st Stage>` | **22 × Ligator** (`800000`) | — |
| `80000100` | `<2nd Stage>` | none | **4** |
| `80000200` | `<3rd Stage>` | none | **5** |
| `80000300` | `<4th stage>` | none | **6** |
| `80000400` | `<Last Stage>` | 6 × Jr. Necki, 3 × Curse Eye, **1 × King Slime** | — |
| `80000500` | `<Bonus>` | 12 × Green Mushroom, 24 × Horny Mushroom | — |
| `80000600` | `<Exit>` | none | — |

Shared `info` on every stage [L]: `returnMap 80000000`, **`forcedReturn 80000600`**,
`fieldLimit 8316`, `fieldScript ""`, `onFirstUserEnter ""`.

**King Slime** `800003`: level **32**, **16 820 HP**, exp 248, PAD 391, MAD 391,
`boss 1`, `bodyAttack 1` [L]. That is the single thing the quest counts.

### The two script hooks — the whole server contract

Only two strings in all seven maps name server behaviour [L]:

```text
80000000  info/onUserEnter = "PQ_01_entered"
80000000..80000400  portal "next00": target map 0, script "PQ_01_nextstage_portal"
```

Every forward portal is a **script portal with no destination** (`target map 0`), one per
stage including the last [L]. So the client asks the server "may I go on, and where?" and
the server answers — which is why a stage cannot be skipped by walking, and why **nothing
happens at all until the server implements these two names**. [I]

The Exit map has only spawn portals [L]: leaving is through Nella, not a portal.

### There are no reactors

`reactor` is present but **empty `{}` in all seven maps** [L]. Instrument checked before
believing the negative, per `CLAUDE.md`: the same dump yields **228 reactor rows across 29
maps** elsewhere, so the tool sees reactors when they exist. This PQ is driven entirely by
**mob kills, NPC conversations and player position** — there is nothing to hit.

### What the `area` rectangles are

Stages 2, 3 and 4 are the only maps in the set carrying an `area` node [L], and they are
the only ones with no mobs [L]. The shapes say what they are:

* **Stage 2 — 4 rects**, tall and narrow (~28 wide × ~180 tall), in two rows of two. Tall
  and thin is a **rope**. [I]
* **Stage 3 — 5 rects**, wide and short (~130 × ~60). Wide and flat is a **platform**. [I]
* **Stage 4 — 6 rects**, small (~45 × ~15), arranged **3 / 2 / 1** as a pyramid
  (y ≈ -133, -184, -235). [I]

## 4. The stage rules — candidate only

`ModernMapleSource`'s `world/partyquest/FirstTimeTogether.java` [C]:

* **Stage 1.** Ligators each drop one **coupon**. Every party member *except the leader*
  talks to Cloto, is told a number, and brings back exactly that many. When all have done
  so the leader clears the stage. The refusal text names the total as *"the number of
  members of your party minus the leader"*.
* **Stages 2 and 3.** *"Out of these ropes/platforms, **3 are connected to the portal**…
  have **3 party members OR 3 items** find the correct ones and hang/stand on them… only 3
  members are allowed on them. Then the leader **double-clicks Cloto to check**."* It also
  warns that hanging too low or standing too near an edge does not count — i.e. the check
  is **containment in a rectangle**, not proximity.
* **Stage 5 (Last).** Kill the boss monsters, gather **passes**, hand them to Cloto.
* Clearing sets a per-instance property `kpq<stage>clear` and opens the portal with *"There's
  a **time limit** on getting there"*; progress is reported as `stage * 20` percent.

It reads the rectangles with a helper literally called **`rectangleStages(stage)`** [C].

### Where the two agree

This is why the [C] rules are worth more than usual here — the v214 script and *this*
client's untouched map data corroborate each other on points neither had to match:

| claim | v214 [C] | this client [L] |
|---|---|---|
| stages 2/3 are solved by standing in marked zones | `rectangleStages()` reads map rectangles | stages 2–4 are the **only** maps with an `area` node |
| "3 of them are correct" | 3 of N | N = **4** (stage 2) and **5** (stage 3) — 3 correct out of 4 and 5 |
| ropes vs platforms | stage 2 ropes, stage 3 platforms | stage 2 rects are **tall/thin**, stage 3 **wide/flat** |
| stage 1 is coupons off Ligators | coupons from Ligators | the **only** stage with mobs before the last, **22 Ligators** |
| the last stage is bosses and passes | passes to Cloto | `4001002` **"Pass"** exists as an item |

**Where they differ, and it matters:** v214 names its hooks `WUK_StageEnter` and computes
the stage as `(fieldID % 10000) / 1000`. This client says `PQ_01_entered` and its stage is
`(map / 100) % 10`. So the *names and arithmetic are ours to define* — do not copy them.

### The one concrete conflict, already in our tree

`data/drops.txt` line 1037 already drops **`4001001` "Coupon"** from Ligator at rate 6 [L].
The client also carries **`4001002` "Pass"** [L]. The v214 text uses *coupon* for stage 1
and *pass* for the last stage [C], which fits having both. Whether our existing Coupon row
was put there for this PQ or by accident is **not established** — it predates this pass.

## 5. What a server would have to own

None of this exists in `crates/world` today [L] — there is no PQ module, no instancing, and
`80000000`–`80000600` are not reachable.

1. **Instancing.** Every stage's `info` is a single shared field. Two parties in
   `80000000` at once would see each other's Ligators. The `kpq<stage>clear` property is
   per-instance in [C], so a party needs its own copy of the field.
2. **Entry.** Lakelis, the leader-only conversation, and the gate the quest states in
   words: four members, all level ≥ 21 [L].
3. `PQ_01_entered` — the stage-1 `onUserEnter`.
4. `PQ_01_nextstage_portal` — one script portal per stage, refusing until the stage's
   clear flag is set, then moving **the whole party**.
5. **Per-stage rules**, of which only stage 1 and the last need mobs.
6. **Position checks** against the `area` rectangles for stages 2–4 — the server must read
   the `area` node, which `tools/dump_portals.py` does **not** currently export.
7. **A time limit and `forcedReturn 80000600`** on failure or disconnect [L].
8. The reward is already expressible: `exp`, `money`, `pop 1` and one of four scrolls [L].

The smallest honest first step is **item 6**: `area` is the one piece of map data this
feature needs that the handbook does not yet carry, and exporting it costs no client run.
