# Jump quests: Forest of Patience, Deep Forest of Patience, Construction Site B1-B3

2026-10-04. The owner: *"research on how Forest of Patience, Deep Forest of Patience, Construction
Site B1 - B3 jump quests suppose to flow, and then we're going to implement them."*

Tags: **[L]** read from this client's own WZ (`Map.wz`, `Npc.wz`, `String.wz`, `Quest.wz`
via `gm-handbook/`); **[R]** the v214 reference source's scripts (a different game version,
whose map, NPC and item ids all differ: a candidate for the flow, never for the numbers);
**[M]** recollection of the old GMS scripts, with no source on disk to check it against.

## 1. What the client gives us, and what it does not

Every NPC in the three courses names a **server-side script** in `Npc.wz/<id>.img/info/script`.
The client has no script bodies, so the flow is the server's to supply **[L]**:

| NPC | name | script | where |
|---|---|---|---|
| 306 | Shane | `patience_enter_306` | Ellinia 10002000 (168, -2879) |
| 314 | Louis | `patience_out_314` | bottom of every Forest of Patience step |
| 315 | A pile of flowers | `patience_flower_315` | top of Forest step 2 (-386, -3940) |
| 316 | A pile of herbs | `patience_herb_316` | top of Forest step 5 (146, -3626) |
| 609 | Mysterious Statue | `mysterious_statue_609` | Sleepywood 10005000 (1061, 255) |
| 610 | Crumbling Statue | `crumbling_statue_610` | bottom of every Deep Forest step |
| 611 / 612 / 613 | A pile of pink / blue / white flowers | `flowers_611..613` | tops of Deep Forest steps 2, 4, 7 |
| 417 | Jake | `jake_417` | Subway Ticketing Booth 10003060 |
| 418 | The Ticket Gate | `the_ticket_gate_418` | Subway Ticketing Booth 10003060 |
| 419 | Exit | `exit_419` | throughout the Construction Site |
| 420 / 421 / 422 | Treasure Chest | `treasure_chest_420..422` | B1 / B2 / B3 Subway Depot |

**No map portal leads into any of the 22 maps** (`portals.txt`: no row outside the courses
targets them) **[L]**. The only ways in are Shane, the Mysterious Statue and the Ticket Gate.

**Each town has a named landing spot beside its NPC [L]**, which is where the way back out
should put a player:

| town | portal | position | NPC beside it |
|---|---|---|---|
| Ellinia 10002000 | `herb` | (290, -2884) | Shane (168, -2879) |
| Sleepywood 10005000 | `forest00` | (1102, 254) | Mysterious Statue (1061, 255) |
| Subway Ticketing Booth 10003060 | `out01` | (287, 186) | The Ticket Gate (272, 187) |

None of the three has a target or a script. They are spawn points that nothing else uses.

## 2. The courses [L]

`returnMap` is the town on all 22 maps. `forcedReturn` is `999999999` except on the three
Subway Depots, where it is Kerning City.

### Forest of Patience, 10002040..10002044 (Ellinia; `fieldLimit` 70)

* **Step 1**: `in00` at the top leads to step 2.
* **Step 2**: **no exit portal**. The pile of flowers (315) at the top *is* the end of the
  first course.
* **Step 3**: `in00` leads to step 4. Step 3 is the start of the second course.
* **Step 4**: `in00` leads to step 5. Seventeen Super Jr. Neckis are the obstacles.
* **Step 5**: the pile of herbs (316) at the top ends the second course.
* **Louis (314)** stands at the bottom of every step.

### Deep Forest of Patience, 10005040..10005046 (Sleepywood; `fieldLimit` 70)

* **Steps 1, 3, 5 and 6**: each has an `in00` to the next step.
* **Steps 2, 4 and 7**: no exit. Each top has a flower pile: pink (611), blue (612) and white
  (613). These are three separate courses, 1-2, 3-4 and 5-7.

  **[I]** Steps 3 and 5 can therefore only be reached through a warp that starts there, so
  the Statue must send each quest to its own course start. With no warp into step 3 or 5,
  the blue and white piles are unreachable.
* **Step 5**: Super Slimes and Super Jr. Neckis.
* **Step 7**: four `tr0x` portals (type 3, touch) at y=500 put a faller back at `st00`.
* **The Crumbling Statue (610)** stands at the bottom of every step.

### Construction Site, 10003100..10003109 (Kerning City; `fieldLimit` 78)

| | Area 1 | Area 2 | Area 3 | Subway Depot (end) |
|---|---|---|---|---|
| B1 | 10003100 | 10003101 | - | 10003102, Treasure Chest 420 |
| B2 | 10003103 | 10003104 | - | 10003105, Treasure Chest 421 |
| B3 | 10003106 | 10003107 | 10003108 | 10003109, Treasure Chest 422 |

* **Each floor ends at its depot.** The depot has no exit portal and is a dead end, so the
  chest or an Exit is the way out.
* **Each depot** carries `timeLimit` 5940 and `forcedReturn` Kerning City. It holds
  Bubblings (B1), Jr. Wraiths plus a Jr. Boogie (B2), or Wraiths plus both Boogies (B3).
* **Exit (419)** NPCs are dotted through every area.
* **B1 Area 1 and B2 Area 1** are full of same-map portals: `in0x`/`out0x` (type 2,
  press-up) and the `go00x` cycle (type 1). **[I]** The client handles a same-map portal
  itself. Nobody has walked these on this server, so a hang on one would be news (§6).

## 3. The quests [L] (`gm-handbook/quests.json`)

| quest | giver | lv | item | count | where it comes from |
|---|---|---|---|---|---|
| 10509 Sabitrama and the Diet Medicine | Sabitrama 603 (Sleepywood) | 25 | 4031025 Pink Anthurium | 1 | Forest step 2, pile of flowers 315 |
| 10510 Sabitrama's Anti-Aging Medicine (after 10509) | Sabitrama 603 | 25 | 4031026 Double-Rooted Red Ginseng | 1 | Forest step 5, pile of herbs 316 |
| 10006 John's Pink Flower Basket | John 100 (Lith Harbor) | 45 | 4031042 Pink Viola | 10 | Deep Forest step 2, pink 611 |
| 10007 John's Present (after 10006) | John 100 | 45 | 4031043 Blue Viola | 20 | Deep Forest step 4, blue 612 |
| 10008 John's Last Present (after 10007) | John 100 | 45 | 4031044 White Viola | 30 | Deep Forest step 7, white 613 |
| 10312 Shumi's Lost Coin | Shumi 403 (Kerning) | 35 | 4031039 Shumi's Coin | 1 | B1 Treasure Chest 420 |
| 10313 Shumi's Lost Roll of Cash (after 10312) | Shumi 403 | 35 | 4031040 | 1 | B2 Treasure Chest 421 |
| 10314 Shumi's Lost Sack of Cash (after 10313) | Shumi 403 | 35 | 4031041 | 1 | B3 Treasure Chest 422 |

The quest text fixes how each course is entered:

* **10509**:
  * *"#p306# from #m10002000# can take me to the right place"* - Shane is the way in.
  * On completion: *"from here on out, I can enter the Forest of Patience without having to
    pay #p306#"*. So a quest holder **pays** Shane, and completing 10509 makes entry free.
* **10510**:
  * *"I should make sure I grab the same exact one once I get to the top. I guess that means
    I have to go back to #b#p306##k"* - Shane again, for the herb course.
  * The completion text repeats the free entry.
* **10006**: *"a #p609# can take me to the flower's location"* - the Mysterious Statue.
* **John's counts**: 10, 20 and 30 must be handed over at once, so **one pile visit gives
  the whole count**. **[R]** agrees: `viola_pink.py` gives 10, `viola_blue.py` 20 and
  `viola_white.py` 30.
* **Shumi's quests** name "Construction Site B1/B2/B3", reached through the subway.

**Shane's own lines [L]:**
* `d0`: *"...I can't let some stranger like you enter my property. I'm sorry, but you'll have
  to leave."*
* idle: *"Are you here at Sabitrama's request?"*, *"Not just anyone can enter this place."*

So in this client a stranger is **refused**. The reference sells anyone over level 25 a way
in; the classic text does not.

**The pile of flowers' only line (315 `d0`) [L]**: *"A strange aura emanates from the pile of
flowers. Unfortunately, there's something strange about the flowers that's preventing them
from being removed."* This reads as the answer to someone **without** quest 10509.

**Other NPC text [L]:**
* Jake (417) `d0`: a warning about the dark. idle: *"You want in? Then purchase a ticket
  here."*, *"No one's allowed in without a ticket."*
* Items 4031036/37/38 are *"Ticket to Construction Site B1/B2/B3"*: one per floor, all
  quest-flagged, sale price 1.
* The Mysterious Statue (609) `d0`: *"(A strange statue. It's hard to tell whether it's
  laughing or crying.)"*

## 4. The flow, NPC by NPC

### Shane (306), Ellinia
**[L]** for who gets in, **[R]** for the fee and the course split (`herb_in.py`).

| who | Shane says / does | where to |
|---|---|---|
| neither 10509 nor 10510 touched | `d0` refusal | - |
| 10509 in progress | yes/no with a fee (**[R]** level x 200 mesos) | step 1 (10002040) |
| 10510 in progress | yes/no with a fee | step 3 (10002042) |
| 10509 complete (and 10510 not in progress) | yes/no, free | step 1 |

**Open:** the course a free entrant gets after both quests are done (step 1, step 3, or a
choice of the two). See §6.

### Louis (314)
At the bottom of every Forest step. Yes/no *"return to Ellinia?"*, then Ellinia `herb`.
**[R]** `herb_out.py`.

### Pile of flowers (315), Forest step 2 top
**[R]** `bush1.py`.
* **With 10509 in progress and no Pink Anthurium held**: yes/no, then 1 Pink Anthurium and
  a warp to Ellinia `herb`.
* **Without the quest**:
  * **[R]** gives 2 of a random gem ore (4020000..4020006) and warps out;
  * **the classic `d0`** says the flowers cannot be taken.

### Pile of herbs (316), Forest step 5 top
**[R]** `bush2.py`.
* **With 10510 in progress**: 1 Double-Rooted Red Ginseng, then a warp out.
* **Without the quest, [R]** gives one of:
  * 2 Diamond Ore (4020007);
  * 2 Black Crystal Ore (4020008);
  * 2 Gold Ore (4010006);
  * 1 earring that is 1032013 in the reference. **In this client 1032013 is Skull Earrings**,
    so that row does not carry over as-is.

### Mysterious Statue (609), Sleepywood
**[R]** `flower_in.py`: *"...Is it okay to be moved to somewhere else randomly just like
that?"* - yes/no.

**[I]** for which step it sends each quest to:

| quest in progress | course start |
|---|---|
| 10006 | step 1 (10005040) |
| 10007 | step 3 (10005042) |
| 10008 | step 5 (10005044) |

The reference only ever sends a player to stage 1 because its maps are laid out
differently; in this client steps 3 and 5 have no way in (§2).

### Crumbling Statue (610)
Yes/no *"go back to where I came from?"*, then Sleepywood `forest00`. **[R]**
`flower_out.py`.

### Flower piles 611 / 612 / 613
Pink / blue / white. **[R]** `viola_*.py`.
* **With the matching quest in progress**: the full count (10 / 20 / 30) when it fits;
  *"make more space in your ETC inventory"* when it does not.
* **Then a warp** to Sleepywood `forest00`, either way.
* **Without the quest**: nothing, then the warp.
* **The reference also refuses a click from too far away**: 225 px horizontally for pink,
  275 px vertically for blue and white.

### Jake (417), Subway Ticketing Booth
Sells the three tickets. **[M]**, unverified:
* B1 at level 20 for 500 mesos;
* B2 at level 30 for 1 200 mesos;
* B3 at level 40 for 2 000 mesos.

The reference has no Jake script.

### The Ticket Gate (418)
A menu of the tickets held. **[M]** One ticket is taken and the player goes to that floor's
Area 1: B1 10003100, B2 10003103 or B3 10003106. With no ticket, the gate says so. The
reference's `subway_in.py` is a modern single-instance version and does not carry over.

### Exit (419)
Yes/no *"leave?"*, then the Ticketing Booth `out01`. **[R]** `old_subway_out.py` has the same
shape.

### Treasure Chests (420 / 421 / 422)
**[M]**
* **With 10312 / 10313 / 10314 in progress and the item not held**: the item.
* **Then a warp** out to the Ticketing Booth.
* **Without the quest**: the GMS chests gave a random item. The list is not recalled and the
  reference has no script.

## 5. Field limits [L]

`fieldLimit` is 70 (`0x46`) on the forests and 78 (`0x4E`) on the Construction Site. Read
against the modern client's `FieldLimit` bits **[R]**:

| bit | meaning | Forests (`0x46`) | Construction Site (`0x4E`) |
|---|---|---|---|
| `0x02` | movement skills | set | set |
| `0x04` | summoning sacks | set | set |
| `0x40` | teleport rocks | set | set |
| `0x08` | Mystic Door | - | set |

The client enforces these itself. `session::summonsack` already relies on bit `0x04`.

## 6. Open questions

### For the owner
* **Shane's fee** (level x 200 per the reference, or another price).
* **The course a free entrant gets** once both Sabitrama quests are done.
* **What a pile or chest gives without the quest**:
  * the reference's ores;
  * the classic "cannot be removed" line;
  * or nothing.
* **Jake's ticket prices and level gates.**

### Settled only by a run
* **Whether a same-map portal sends anything at all.** B1 Area 1 has thirteen of them.
* **What the depots' `timeLimit` 5940 does in the client.** It may draw a clock or eject on
  its own.

## 7. Where the server is today [L]

* **NPC clicks.** None of the eleven NPC templates above has a branch in `on_npc_click`
  (`session/npc.rs`). They fall through to their `d0` line, or to the "no dialogue"
  placeholder when they have none. So all three courses are **unreachable today** except
  through `!warp`, and once inside there is no way out but a return scroll or `!warp`.
* **Templates to copy.** `session/hotel.rs` already does menu, fee and warp, and
  `job_test_exit_for` already does a warden.
* **The quests themselves** (start, hand-in, rewards) are data-driven and need nothing
  course-specific.

## 8. Decided by the owner, 2026-10-04

* **Shane's fee:** none. A stranger to quest 10509 is still refused.
* **Shane's door:** a menu of the two Forest courses; the Statue's, of the three Deep Forest ones.
* **What a goal gives:** the **Jump Quest Reward** at every goal, the chests included, with
  the quest item added while the quest is in progress. The reward is one prize each from the
  Companion's Magic Box's use and scroll slots. *"the chest also gives jump quest rewards in
  addition to the quest item."*
* **Jake:** B1/B2/B3 at level 20/30/40 for 500/1 200/2 000 mesos.
* **The pity timer:** an hour per player for a **quest entry** only (*"it should not be active
  when the player is completing additional attempts for just the jump quest reward"*).
  * It is kept across a log out or disconnect.
  * It is stopped only by a warden, by finishing, or by `!skipjq`.
  * A leftover row is removed only once its player is outside that course.
  * After the hour, a yellow reminder comes every 5 minutes.
  * `!skipjq` then leaves with the quest item and nothing else.
* **Door menus:** a course's quest in progress limits the door to that course. Between two
  quests of a chain, only the completed courses are offered. Every course is offered once all
  are completed. A player who never took any of the door's quests is turned away, as a
  stranger: "authorized personnel only".
* **The Pet-Walking Road** (10001052) joins the set, with no pity timer:
  * Trainer Bartos (222, bottom) gives Bartos's Letter (4031035) to a player whose pet is out.
  * Trainer Frod (223, top) takes it for +20 closeness - the owner's number; the reference
    `pet_letter.py` leaves only a comment there - plus the Jump Quest Reward.
  * There is no warp: the hidden `h005` beside Frod leads back down to `h006` by Bartos
    **[L]**.
