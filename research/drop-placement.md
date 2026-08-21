# Drops that land in walls: the floor, the rule, and the two numbers behind it

**2026-08-21.** Item 1 of `STATUS.md`'s "what to do next, in order".

> ## BUILT AND NOT WIRED
>
> `crates/world/src/footholds.rs` exists, is tested (18 tests, and one of them runs against
> the real 94 089-row table), and **nothing calls it**. On screen it is identical to not
> existing. The exact edits are in **WIRE IT LIKE THIS** at the bottom of this file; they are
> in `crates/world/src/session/` and `crates/world/src/bin/world_server.rs`, which the
> coordinator owns.

---

## 1. What is actually wrong today

The server states a drop's resting position and the client puts it there. **It does not
simulate a fall.** So whatever the server says is where the item lies for good.

`crates/world/src/session/combat.rs`, the kill path:

```rust
let (mob_x, mob_y) = (x, y);
for (i, r) in rolled.into_iter().enumerate() {
    let offset = (i as i16 - (n - 1) / 2) * crate::drops::DROP_STAGGER_PX;
    let x = x.saturating_add(offset);      // <- x is shadowed
    ...                                     //    y is NEVER touched
```

Every drop keeps the corpse's height however far sideways it went. That is invisible on flat
ground and wrong everywhere else - and "wrong" here does not mean cosmetic:

**The client's pick-up sweep is a box of `x-0x19..x+0x19` by `y-0x32..y+0x0a` around the
player** (`research/item-drop.md`, read off the listing, **[L]**). An item more than **10
pixels below the player's feet** is outside that box and cannot be collected however long
anyone stands on it. On screen that is the same picture as the item not existing.

### How often, measured, with no client run spent

Replaying today's placement over **every mob spawn point** in `gm-handbook/mobs.txt` at the
stagger offsets the kill path actually produces (`0`, `+/-20`, `+/-40`), and interpolating the
real surface at each staggered x:

| | |
|---|---|
| placements simulated | 39 712 |
| already on a surface | 28 117 |
| **in mid-air or inside terrain** | **11 487** (29%) |
| no walkable surface at that x at all | 108 |
| **missing by more than the client's own 10 px** | **3 255** (8.2%) |
| median miss / p90 / worst | 5 px / 33 px / **2 915 px** |

The method is **[L]** - both files are generated from the client's own `Map.wz`. The number
is an **[I]** estimate of scale rather than a census of real kills: a mob walks before it
dies, so its spawn point is a stand-in for its death position.

---

## 2. A smaller y is HIGHER UP - measured, not assumed

Getting this backwards inverts the whole rule silently, so it was not taken on trust. Three
independent readings, all off the client's own data:

**(a) The client's own name for the smaller number is "Top". [L]**
Every map image's `info` node carries `VRTop`/`VRBottom`. **Enumerated over all 426 map
images**, not sampled:

```text
maps examined                : 426
VRTop <  VRBottom            : 413
VRTop >= VRBottom            : 0
no VR bounds at all          : 13
```

Read with `target/release/wz-dump cat <archive> <image>.img`. Map 1 for instance is
`VRTop -310, VRBottom 675`.

**(b) The pick-up box is asymmetric, and the long side is the body's side. [L]**
`x-0x19..x+0x19` by `y-0x32..y+0x0a`: 50 pixels one way, 10 the other, around a position that
is at the player's feet. The 50 is where the body is, and a body is above its feet.

**(c) A hovering NPC hovers toward the smaller number. [L]**
Of 10 236 `life` entries that name a foothold, **10 235 sit exactly on it** (interpolated at
the entry's own x, within 1 px). The one exception is NPC template **419 on map 10003108**,
placed at `cy` 1273 on foothold 263 whose y is **1573** - anchored to the ground and drawn 300
px above it. "Above" is 300 *less*.

> (c) is also what licenses the whole approach: a foothold's y at a given x **is** the y a
> body rests at, so putting a drop there puts it exactly where a player's feet will be.

**So: to fall is to gain y, and "the platform above" has the smaller y.**

---

## 3. The floor data

`gm-handbook/footholds.txt`, from `tools/dump_portals.py` out of the `foothold` nodes.
Regenerate with `python tools/dump_portals.py` **from the repo root**
(`cd C:\MapleCW` first).

| | |
|---|---|
| segments | **94 089** across **426** maps |
| walls (`x1 == x2`) | **31 220** - never a landing surface |
| flat floors | 35 292 |
| **sloped floors (`y1 != y2`)** | **27 577** |
| degenerate (a point) | 0 |
| duplicate ids within one map | 0 |
| coordinate range | x `-5570..6338`, y `-4187..3900` - all inside `i16` |

**A third of the floor in this game is sloped.** Reading `y1` instead of interpolating would
be right 35 292 times and wrong 27 577 times, usually by less than an icon's height - which is
the exact shape of a bug that reads as bad luck. `Foothold::surface_y` interpolates.

`prev`/`next` are parsed and kept and **nothing reads them**. Placing a drop is a question
about one vertical line, not about a walk.

---

## 4. The rule

The owner:

> *"Problem with mob drops, if they were to drop into the wall, please do not make it so that
> it cannot be retrieved. If it clips into the wall, it should fall onto the platform above it
> (since item will arc upwards about the same distance of a player's jump). If it won't reach
> that platform, it should fall down to the next available platform below."*

Implemented as **one comparison**, not a chain of special cases:

> Take the **nearest walkable surface** on the vertical line through `x`. If the nearest is
> *above* the point, accept it only when the rise is within `JUMP_HEIGHT_PX`; otherwise take
> the nearest one *below*.

| the point is | nearest above | nearest below | answer |
|---|---|---|---|
| standing on flat ground | a ceiling, far | itself, **0 px** | stays exactly where it is |
| past the edge of a ledge | none, or far | the floor beneath, 60 px | falls 60 - items do fall |
| clipped into a step | the step, 30 px | the cliff bottom, 200 px | **lifted 30 onto the step** |
| clipped into a cliff face | the clifftop, 200 px | the cliff bottom, 400 px | out of reach: **falls 400** |
| between two floors | 40 up | 10 down | falls 10 - the nearer, and the physical one |

Four properties worth stating because each is a decision:

* **A tie falls.** `da < db`, not `<=`. Dropping is what an object does when nothing stops it,
  and a rule that lifted on a tie would lift an item that was already on the ground.
* **x never changes.** The arc decided where it went sideways; this decides only how far it
  sank or rose.
* **Walls need no detection beyond being excluded.** A point buried in a cliff face has the
  clifftop above it and the cliff bottom below it, and those two distances are what the rule
  already compares. `surface_y` returns `None` for a vertical segment, so a wall never enters
  either candidate list.
* **`None` means "keep what you had".** Returned for a map not in the table, an empty table,
  an x with no walkable surface at all, and a lone platform above that is out of jump reach.
  In every one of those the caller's own fallback is better, because that fallback is where
  the mob was standing and **a mob stands on a foothold**. `rest_at` takes the fallback as a
  parameter so a call site cannot forget it.

---

## 5. Where the jump height came from

**77 pixels**, and it is **not** from a fan site and **not** from `ModernMapleSource` - which
`CLAUDE.md` scores at 1 of 8 and which was not consulted for this at all. It is from **this
client's own physics table**:

```text
target/release/wz-dump cat client-patched/Data/Map/Map_000.wz Physics.img

  "gravityAcc": 2000.0,
  "fallSpeed":   670.0,
  "jumpSpeed":   555.0,
  "walkSpeed":   125.0,
  ...
```

A body launched at `v` against a constant `a` peaks at `v^2 / 2a`:
`555 * 555 / (2 * 2000)` = **77.006 px**.

* **`jumpSpeed` and `gravityAcc` are [L]** - literally in the client's `Map.wz`.
* **The 77 is [I], derived from them** by the closed-form projectile height. The client
  integrates per frame, so its real apex is a hair under 77. This is a threshold for "could a
  player have got up there", not a simulation.
* It lives in **one named constant**, `footholds::JUMP_HEIGHT_PX`, and a test checks the
  boundary from both sides so changing it cannot pass silently.

**The unit check, because `CLAUDE.md` says to make one.** These are pixels per second and
pixels per second squared: `walkSpeed` 125 and `fallSpeed` 670 are a plausible walk and a
plausible terminal velocity in those units and in no others, and 77 px is a little over one
character's height - which is what a jump in this game looks like. A wrong unit here would
have produced an absurd number, not a plausible one.

---

## 6. WIRE IT LIKE THIS

Four edits, all in files the coordinator owns. `crates/world/src/footholds.rs` and the one
`pub mod footholds;` line in `crates/world/src/lib.rs` are already in place.

### 6.1 `crates/world/src/config.rs` - carry the table

In `pub struct Config`, beside `pub fields:`:

```rust
    /// Every map's floor, from `gm-handbook/footholds.txt`, so a drop lands somewhere a
    /// player can reach it. See `crate::footholds` - an empty table means every drop keeps
    /// the position the caller already had, which is today's behaviour.
    pub footholds: crate::footholds::Footholds,
```

and in `impl Default for Config`, beside `fields: std::collections::HashSet::new(),`:

```rust
            footholds: crate::footholds::Footholds::default(),
```

### 6.2 `crates/world/src/bin/world_server.rs` - load it, and be LOUD on stdout

Beside `let mut fields_path = ...`:

```rust
    let mut footholds_path = PathBuf::from("gm-handbook/footholds.txt");
```

an argument beside `"--fields"`:

```rust
            "--footholds" => value().map(|v| footholds_path = PathBuf::from(v)),
```

a `USAGE` line beside the `--fields` one:

```text
  --footholds PATH   map floor geometry, from tools/dump_portals.py
```

and the load itself, beside `config.fields = ...`:

```rust
    // The floor. A missing file degrades to today's behaviour - drops land at the height the
    // mob died at - and the banner says which of the two states we are in, on stdout, in both
    // cases. `Config::map_exists` was fail-open on an empty table with its warning on
    // stderr, where nothing reads it, and that cost a client run and a crash on 2026-08-20.
    config.footholds = world::footholds::Footholds::load(&footholds_path);
    println!("{}", config.footholds.banner());
    for line in config.footholds.problems() {
        println!("maplecw-world: footholds: {line}");
    }
```

`banner()` already prefixes itself with `maplecw-world: footholds:` and prints a **different**
line in each state. Print it unconditionally - a banner that is silent when things are fine
cannot be told apart from one that is not being printed at all, which is precisely how the
`!map` guard went missing.

### 6.3 `crates/world/src/session/combat.rs` - the kill path

In `drops_from_kill`, **one line**, immediately after the existing stagger:

```rust
        for (i, r) in rolled.into_iter().enumerate() {
            let offset = (i as i16 - (n - 1) / 2) * crate::drops::DROP_STAGGER_PX;
            let x = x.saturating_add(offset);
            // **Put it on the floor.** The stagger moves the item sideways and nothing moved
            // it vertically, so on any ground that is not flat it ends up in mid-air or
            // inside terrain - and the client's pick-up box reaches only 10 px below the
            // player's feet, so such an item is drawn and uncollectable. The fallback is the
            // corpse itself, which is certainly a floor because a mob was standing on it.
            let (x, y) = self.config.footholds.rest_at(map, x, y, (mob_x, mob_y));
```

That is the whole change. `source_x`/`source_y` stay `mob_x`/`mob_y` - the arc still starts on
the corpse, and now it ends somewhere real.

`y` is shadowed for the rest of the loop body, which is what the two `x`/`y` fields of
`DropFromMob` already read - **no other line in the loop changes.**

Optional, and worth it on the first run: `Footholds::landing(map, x, y)` returns the same
answer with `foothold`, `how` and `moved` on it, and `Landing::what()` formats the log line.
Logging the ones where `moved != 0` turns "did the fix do anything" into a `world.log` grep
instead of a second launch.

### 6.4 `crates/world/src/session/ground.rs` - an item dragged out of the bag

In `on_drop_request`, after the existing `last_position` check:

```rust
        let Some((x, y)) = self.last_position else {
            return self.refuse_drop(m, "...");        // unchanged
        };
        // `last_position` is the last movement path point, which can be **mid-jump**. An item
        // left hanging in the air where the player happened to be is exactly as uncollectable
        // as one inside a wall. The fallback is the player's own position, so a player already
        // standing on the ground sees no change at all.
        let (x, y) = self.config.footholds.rest_at(chr.map_id, x, y, (x, y));
```

### 6.5 What to watch on the run that tests it

One variant at a time. The discriminating test is **a mob killed near a ledge or a step**, not
on flat ground - flat ground looks identical before and after, which is why this went
unnoticed.

| watch | means |
|---|---|
| the startup banner names a foothold count | the table loaded; if it says `NONE LOADED`, nothing below is being tested |
| an item dropped by a mob on a slope or step can be walked over and picked up | the fix works |
| an item still hangs in the air | the y direction or the surface lookup is wrong - **read the `world.log` line, which names the foothold** |
| an item lands on the level above the mob | the lift fired; check it was within 77 px and not a sign flip |

---

## 7. What this does NOT do, stated plainly

* **It does not move a drop sideways**, so an item can still land on a floor a wall separates
  it from - reachable, but by a walk rather than a step. The owner's rule is about height and this
  implements exactly that.
* **It does not know about ropes, ladders or one-way platforms.** Those are separate WZ nodes
  and are not in `footholds.txt`.
* **It changes nothing about who may pick a drop up, or when it expires.** Ownership and
  `DROP_LIFETIME_MS` are `crates/world/src/drops.rs` and are untouched.
* **Nothing authenticates.** The game socket still carries no credentials, and none of this
  changes that.
