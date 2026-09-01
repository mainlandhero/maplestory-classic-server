# Making mobs the same for two players on one map

Working for `crates/world/src/mobshare.rs`. Labels are the project's: **[L]** read off this
client's listing, its WZ or a capture; **[D]** derived from two or more [L] facts; **[I]**
inferred - policy nothing on this machine can confirm.

> **Nothing here has ever been on a wire between two players.** Two clients have never been
> connected to this server at once. Every claim about what a second player *sees* is [I] on
> that point, however well-read the packet is.
>
> **WIRED 2026-09-01.** This banner used to end *"§9 is the wiring and it is not applied"*.
> It is applied: `crates/world/src/mobshare.rs`'s own header lists the call sites, and §9 is
> now a record of where each piece went plus the three places this document's recipe turned
> out to be wrong when someone tried to follow it. **§1's four bugs are fixed** and are left
> in the present tense below only as a description of what was found - each now names the
> test that pins it.
>
> What has NOT changed is the first sentence of this banner. Wiring it does not put it on a
> wire, and no second client has ever seen any of it.

---

## 0. Answer up front

**Exactly one connection controls each mob; control is claimed by a session for itself, never
handed to another session; and it is never revoked while its holder is still on the map,
because the client's only revoke is a despawn.**

Everything else falls out of that. Damage, death, respawn and movement are published to the
map; grants and acknowledgements are private to one connection; drops are private to the top
damager, and the party rule is one predicate.

| event | to the acting session | to everyone else on the map |
|---|---|---|
| field entry | `0x03C6` per mob, `0x03D2` for the ones it claimed | `0x0224` (already wired) |
| respawn tick | `0x03C6` + `0x03D2` for the mobs it drained | **`0x03C6`** |
| a swing | `0x007C` MP, quest rows, drops | `0x029E` (already wired) |
| a hit that wounds | `0x03F0` | **`0x03F0`**, identical bytes |
| a hit that kills | `0x03D1`, `0x046E` per drop, `0x007C` EXP | **`0x03D1`**. No drop, no EXP packet |
| a controller's `0x02FF` | `0x03E4` | **`0x03D9`**, superseded per mob |
| leaving the map | nothing | `0x0225` (already wired) |

The four packets in bold are the whole of the mob half. Three of them already exist as bytes
in `crates/net` and are already built by `session/combat.rs` for the attacker's own reply -
the change is a second destination, not a second builder. The fourth, `0x03D9`, is built and
tested in `net::mobmove::mob_move_broadcast` and has had no production caller since the day it
was written.

---

## 1. What is wrong today, from this repo's own code

Not suspected. Read out of the files named.

* **Every session is granted every mob.** `session/field.rs::on_field_entered` pushes a
  `MOB_CHANGE_CONTROLLER` for every mob in `Fields::mobs_on`, unconditionally, to whoever
  walked in. `fields.rs` has no controller registry.
* **The client rolls the wander itself.** `research/mob-behaviour.md` §5.1: mob vtable slot 19
  `FUN_141c8d1b0` builds a list of 12-byte path elements out of the random source
  `FUN_142f04924`, one call per element field, and hands them to the move builder at slot 22.
  **[D]** Two grants are two independent dice rolls. The screens diverge on the first step and
  never reconverge.
* **A respawn is unicast.** `Fields::due_respawns` does
  `std::mem::take(&mut field.pending)` and partitions. Whichever session's `tick` runs first
  takes the due spawn points; the other session is never told the mob exists. **[L]**
* **Two controllers are two writers of one position.** `Fields::note_position` is called from
  `session/combat.rs::on_mob_move` with no check of who is reporting. Last write wins, and
  that position is what a drop lands on. **[L]**
* **An expired drop fades on one screen.** `DropTable::sweep` removes the drop from the
  shared table and returns the `0x046F` to the calling session only. **[L]**
* **`Bus::publish` has exactly two production callers**, both in `session/multiplayer.rs`:
  `publish_user_move` and `publish_user_attack`. Nothing about a mob or a drop crosses.

---

## 2. Why control is sticky: the only revoke deletes the mob

`net::mobmove::CONTROL_RELEASE` is level `0`. `141d30ef5 TEST EBP,EBP / JE 141d30f1c` takes
the zero branch straight into the pool's erase path - `FUN_141d51320(pool+0x38, …)`,
`FUN_141d51670(pool+0x68, &id)`, `FUN_141d51700(pool+0xa8, &id)` - and the body stops after
five bytes. **[L]**, and `net::mobmove::mob_release_controller`'s own doc says it: *"Named for
what it does rather than for the field it sets, because 'release control' is what the level
byte looks like and deleting the mob is what happens."*

`research/mob-behaviour.md` §13 already flagged the consequence as the one open scheduling
question: *"A real server rotates control as players move, and `0x03D2` with level 0 despawns
rather than releasing, so a naive rotation deletes the mob."*

So there is no "you are no longer the controller, keep drawing it" packet. The alternatives
are:

| | cost |
|---|---|
| **never rotate** (chosen) | a mob keeps its controller until that client leaves the field |
| despawn + `0x03C6` re-spawn | the mob pops off and back onto the old controller's screen, and its animation state resets. Two packets and a visible artefact, for no gain a player can name |
| rotate by proximity | the above, on every walk past |

**Chosen: never rotate, and never send `CONTROL_RELEASE` at all.** A controller loses its mobs
only by leaving the field - and a client that leaves a field has already destroyed its own mob
pool, because the `SetField` handler does it. So the losing side of a handover costs **zero
packets**.

---

## 3. Every transition is a session claiming for itself

`Controllers::claim_uncontrolled` is a test-and-set under one lock. A session calls it for
**itself**, on its own map, in its own thread.

```text
  A enters an empty map     claim_uncontrolled -> [2000, 2001, 2002]    3 x 0x03C6 + 3 x 0x03D2
  B enters the same map     claim_uncontrolled -> []                    3 x 0x03C6, no grant
  A logs out                release_all(A)     -> 3 freed               nothing
  B's next tick             claim_uncontrolled -> [2000, 2001, 2002]    3 x 0x03D2
```

**No control packet is ever addressed to a connection other than the one building it.** That
is what makes this cheap: no new bus channel is needed for control, and "two controllers"
cannot be reintroduced by a routing mistake, only by deleting the registry.

The cost is **one tick of latency** on a handover. The mobs a departing controller held are
orphaned until some other session's `tick` claims them, and an orphaned mob does not move
(nobody reports, so `note_position` is not written and `LiveMob::as_seen` keeps handing out
the last known place). The tick is the 100 ms wakeup `crate::broadcast` describes. **[I]** as
to what a tenth of a second of stillness looks like; nobody has watched it.

### 3.1 A mob with no controller

Alive, drawn on every screen that got its `0x03C6`, standing still. That is the same state a
map with nobody on it already has, and it is the owner's own model for it: *"When the user leaves
the map, all of the mobs should persist in their current location."*

`may_report_movement` refuses a `0x02FF` for an uncontrolled mob. It cannot legitimately be
reporting: the move sender is only reached once slot 8 has been switched on by a `0x03D2`
(`research/mob-behaviour.md` §4), so a report for one is either a stale grant from before this
change or an invented packet. Nothing on this socket authenticates anybody.

### 3.2 `Drop` is the single point of failure

A session that vanishes without `release_all` leaves mobs claimed by a dead id, and nobody
will ever claim them again - monsters that have stopped moving for no visible reason.
`Bus::part` is already in `Drop for Session`; `release_all` goes on the line beside it, not in
a different function.

`Controllers::reconcile` is the belt to that braces, and it is a **separate** call rather than
part of `claim_uncontrolled`. That split came out of a failing unit test: folded together, a
caller passing a *partial* `alive` list silently frees every **other** session's mobs on the
map, and the symptom is another player's monsters freezing on a screen nobody is looking at. A
destructive step belongs where the caller can see it.

---

## 4. Damage and death, broadcast

`session/combat.rs::on_attack` already builds exactly the right bytes and sends them to one
person. `net::combat::mob_hit_replies(object_id, &hit, max_hp)` returns:

* still alive -> one `0x03F0 MobHpChange` = `u32 objectId, u32 hp, u8 showBar`, **9 bytes**;
* killed -> one `0x03D1 MobLeaveField` with `deathType = 1` (`death::ANIMATED`), **14 bytes**,
  and no `0x03F0`, because a bar update for an object being torn down is a write to a pool
  entry the client is removing.

Both are recipient-independent, so the fix is `Bus::publish` with the **same `(opcode, body)`
the attacker gets**.

> **Publish the same body. Do not recompute it.** `0x03F0`'s `hp` field is a **percentage**,
> 0..100 - `net::combat::hp_percent`, `research/mob-hp-bar.md`, and `CLAUDE.md`'s "the unit,
> not the arithmetic". The absolute went out once and drew a 45-HP snail at 27 %. A second
> call site that looked `max_hp` up its own way is exactly how that comes back. One body, two
> destinations.

The observer's own damage numbers do **not** come from `0x03F0`. They come from the attack
rebroadcast `0x029E`, which already carries the damage - `session/multiplayer.rs`'s test
asserts `u64` 19 at body offset 69, *"the damage the attacker's own client computed, absolute
and unscaled"*. So for an observer, `0x03F0` is the health bar and nothing else.

### 4.1 What the receiving client needs

The mob in its pool. It is: every client on the map got a `0x03C6` at its own field entry, or
from the respawn publish. If it somehow is not - a player who joined mid-fight, or the narrow
duplicate race in §9 item 3 - the packet is dropped in silence rather than faulting. Checked
today with `tools/listing.py`, whose documented positive control (`0x140304100` -> `raw`,
`u8`, `u8`, then a run of `u16`) was run first and matched:

```text
0x03F0, 0x03D9, 0x03E4    second dispatcher FUN_141d32b30
  141d32b4d  call 0x1406e8c20        READ u32   the object id
  141d32b57  call 0x141d2efc0        the pool lookup
  141d32b5f  test rax,rax
  141d32b62  je   0x141d33432        not found -> the epilogue
  141d33432  lea r11,[rsp+0x70] / mov rbx,[r11+0x20] / mov rsp,r11 / pop / ret

0x03D1                    FUN_141d33c70, which reads the WHOLE body first
  141d33d02  mov  r8,[rbx+0x68]      the bucket array
  141d33d09  je   0x141d34254        null            -> the epilogue
  141d33d26  je   0x141d34254        empty bucket    -> the epilogue
  141d33d3f  jmp  0x141d34254        chain walked out -> the epilogue
  141d34254  mov rbx,[rsp+0xa8] / add rsp,0x60 / pop x7 / ret
```

**[L]** for all six exits. An unknown object id costs a body read and a return. It does not
fault and it does not desynchronise the stream, which is the property that makes a map-wide
publish safe at all.

---

## 5. Movement: `0x03D9` is needed, and it is the cheap half

**Yes, it is still needed with one controller.** One controller means exactly one client is
simulating; the others are told nothing at all unless `0x03D9` goes out, so a mob that walks
on A's screen stands still on B's. That is the same divergence, in a quieter form.

`net::mobmove::mob_move_broadcast` is built, tested against two real captured `0x02FF` bodies,
and has **no production caller**. `research/mob-behaviour.md` §12.1 states the rule this
wiring has to honour and why:

> **Never send `0x03D9` to the client that sent the `0x02FF`.** `FUN_141c813b0` overwrites the
> mob's position, animation and `mob+0xcd0` from the packet - state the controller owns.

`Bus::publish` excludes the sender, and the sender **is** the controller (it is handling that
controller's own `0x02FF`), so the rule is satisfied by construction rather than by a second
check. That is only true if the controller gate runs first, which is why `may_report_movement`
comes before everything else in the handler.

### 5.1 Cost

One `0x03D9` per inbound `0x02FF` per **other** client on the field. Body is
`25 + 14 + 21n` bytes - 60 for a one-element path, 123 for a four-element one. `0x02FF`
arrives roughly once per mob per path, and 164 117 of them are in the archive, so on a 30-mob
map with two players this is one extra 60-120 byte packet per mob per step, in one direction.
Nothing here needs a timer: `research/mob-behaviour.md` §13 - both packets are **reactive**,
and no periodic send should be added speculatively, because the client is the clock.

### 5.2 It supersedes, and it is the only thing that does

`Bus::publish`'s fourth argument is documented as *"the character the packet is about"*, and a
mob object id is not a character id. The namespaces overlap - character ids start at 200, mob
object ids at 2000 - so this is a real question rather than a pedantic one.

It is safe **because the bus matches on the pair `(opcode, key)`** (`Inner::post`:
`q.supersedes == Some(c) && q.reply.opcode == reply.opcode`) and `0x03D9` is a mob-only opcode
that no character-keyed packet can ever share. That argument has to hold for every opcode
given a key, which is why only one is given one.

Movement is superseded because a mob wanders for as long as it is alive: an observer who stops
reading accumulates `0x03D9` **without bound**, and a position that has been overtaken is
worth nothing. `0x03F0` is not, because it is bounded by the length of a fight, it arrives
beside a `0x029E` that is already unsuperseded, and it carries an event as well as a state -
the same packet pushes into the mob's floating-number list at `mob+0x6d8`
(`net::combat::mob_hp_change`, **[L]**), so coalescing would drop numbers.

**Both readings are defensible and neither has been on a screen.** [I].

---

## 6. Drops: per client, to the top damager

The owner, 2026-09-01: *"the drops can remain per client. If multiple clients hit the mob, the one
who dealt the most damage (without counting over-damage) will see the drops."*

### 6.1 "Most damage without over-damage" already exists

`LiveMob::credit` does `let landed = damage.min(self.hp);` before the subtract, and
`LiveMob::shares()` returns the split highest-first with exactly one `majority`, ties broken by
first blood (a **stable** sort, deliberately - `fields.rs` says so). That is the same ranking
the EXP split and its white/yellow line already use.

`mobshare::drop_audience` reads it and **adds no arithmetic**. A second implementation of "most
damage" is a second thing to get wrong, and the first one is already paying out experience.

### 6.2 The client cannot help, so this is entirely the server's

`net::drops`: `ownType` is read at `0x1417a3539` into `drop+0x70` and **never tested again**.
**[L]** So visibility is exactly the question of who is sent a `0x046E`, and nothing else.
`own_type_for` sets the byte truthfully anyway - the next person to read a capture will check
it against the rule, and a lie there costs an hour.

### 6.3 The four cases

| case | answer |
|---|---|
| **solo kill** | one contributor, who is the killer. Unchanged from today |
| **tie** | first blood, inherited from `shares()`'s stable sort. The same fight scores the same way twice |
| **top damager has left the map** | `drop_audience` returns a **ranked list**, and the caller walks it, stopping at the first candidate who is on this map. The killer is always in the list and always on the map, so the walk terminates |
| **nothing credited at all** | `Fields::hurt` returns `Died(vec![])`. Falls back to the killer, matching `award_kill_experience`, which already treats that case as "pay the killer in full" |

A ranked list rather than one winner, because an item nobody can see is indistinguishable from
no item - which is `session/combat.rs`'s own sentence about a drop placed out of reach.

### 6.4 What this needs that does not exist

The winner may be a **different connection** from the killer, and there is no way to send a
finished packet to one named character:

* `Bus::publish` is addressed to a **map**, and would show the loot to everyone;
* `Bus::send_to_character` is addressed to a character but carries an `Event`, not bytes -
  deliberately, and `broadcast.rs` says why: `0x007C` carries the recipient's own new total,
  *"a finished EXP packet cannot be correct for anyone but its author"*.

A `0x046E` is not like that: it is identical for every viewer. So the honest addition is one
method, ten lines beside `send_to_character`, and it belongs to the coordinator:

```rust
pub fn publish_to_character(&self, character: u32, map: u32, reply: Reply) -> bool
```

The `map` is not optional. A `0x046E` names a position in the field the recipient is standing
in, so delivering one to a character who has walked away would put a phantom item on a map it
was never dropped on.

**If the coordinator would rather not add it**, the fallback needs no shared-file change: give
the drop to the highest-ranked candidate *who is this session*, else to the killer. That is
"the top damager sees the drops unless the killing blow came from somebody else", which is not
what the owner asked for, and it should be said out loud rather than shipped quietly.

### 6.5 Two consequences worth naming rather than discovering

* **`drops::OWNER_LOCK_MS` becomes unobservable.** It says a drop may be taken by anyone after
  fifteen seconds. With visibility restricted to the audience, a player who may take a drop
  they cannot see will never ask for it. The rule does not become *wrong*; it becomes dead.
  Aligning the two is a `drops.rs` decision.
* **`DropTable::field_entry` re-sends the whole floor** to whoever walks in, and
  `DropTable::sweep` fades a drop for whichever session ticked. Both have to be filtered by
  `may_see_drop`, or per-client drops leak on re-entry and ghost on expiry. §9 item 7.

---

## 7. The party seam

One predicate:

```rust
pub fn may_see_drop(owner: u32, viewer: u32, party: &Party) -> bool {
    owner == viewer || (party.contains(owner) && party.contains(viewer))
}
```

`Party::solo(character)` is the whole of today's rule. When the party system lands, the **only**
change is the call site that builds the value: `Party::of(character, members)` instead of
`Party::solo(character)`. Nothing in `may_see_drop`, in `drops.rs` or in `net::drops` moves,
and `own_type_for` starts returning `OWN_TYPE_PARTY` for free.

`Party::of` folds the owner in whether or not the caller listed them, so a party that has
forgotten its own member cannot hide a drop from its owner.

**No party system is built here.** A sibling agent owns that.

The EXP side already has its own note and does not go through this predicate:
`session/combat.rs::award_kill_experience` pays every contributor by damage share, and
`research/exp-sharing.md` records the 70/30 party rule and the `You received party EXP` line
that are not implemented.

---

## 8. What is decided rather than measured

Six things, all [I], all reversible, and all somewhere the owner could reasonably have gone the
other way:

1. **Control never rotates.** Forced by the despawn, but "never" is still a choice - a
   despawn-and-respawn rotation is available and was rejected for a visible pop.
2. **The first session to learn a mob needs a controller takes it.** First arrival takes the
   whole map; a respawn goes to whichever session ticked first, which spreads control over
   time. The alternative - balancing the map's mobs across the sessions on it - was rejected
   as harder to read in a log, and its one real advantage is noted: with one session holding
   everything, a single stalled client freezes every monster on the map.
3. **`0x03D9` supersedes and `0x03F0` does not.** §5.2.
4. **The drop goes to the top damager, falling back down the ranking to the killer** rather
   than always to the killer, or always to the top damager even when they cannot see it.
5. **A drop is visible to exactly the people who may take it.** The alternative is to keep
   `OWNER_LOCK_MS` meaningful by showing a drop to everyone and refusing the pick-up, which
   contradicts *"the drops can remain per client"*.
6. **A non-controller's `0x02FF` is ignored, not answered.** `CLAUDE.md`'s always-answer rule
   is about a request that latches the UI; `0x02FF` is volunteered, and `on_mob_move` already
   returns nothing for an unparseable body. Answering it would pump a second simulation, which
   is the thing being removed.

---

## 9. WIRE IT LIKE THIS

The full patch with anchors is the `WIRE IT LIKE THIS` comment block at the bottom of
`crates/world/src/mobshare.rs`. In outline, all of it in files this agent does not own:

0. `fields.rs` - a `controllers: crate::mobshare::Controllers` field and a `controllers()`
   accessor, beside `bus` and for the same reason. Its lock is a leaf.
1. `session/field.rs::on_field_entered` - `reconcile`, then `claim_uncontrolled`, then send
   `0x03C6` for **all** mobs and `0x03D2` only for the claimed ones.
2. Every `leave_the_field()` call site and `Drop for Session` - `release_all`.
3. `session/combat.rs::spawn_due_mobs` - publish the `0x03C6` to the map, `claim_one`, grant
   only if the claim won.
4. `session/combat.rs::on_mob_move` - gate on `may_report_movement`, then publish `0x03D9`.
5. `session/combat.rs::on_attack` - publish the same `mob_hit_replies` bytes; `forget` on the
   death branch.
6. `session/combat.rs::drops_from_kill` - take the ranked audience instead of `chr_id`; needs
   `Bus::publish_to_character` (§6.4).
7. `session/field.rs` and `session/mod.rs::tick` - filter the floor and route the fade by
   `may_see_drop`.

---

## 10. What a run would settle, and what it would not

Two clients on one map, one map with mobs, one player attacking. **One variant at a time.**

| watch | if it holds | if it does not |
|---|---|---|
| B's screen while A kills a mob | the bar moves and the mob dies on both screens | count the event in **two** logs: `world.log` says the server published it, `client-patched\maplecw-hook.log` says the client dispatched it - and that line is written on **return**, so a missing one means the handler was entered and never came back |
| the mob's position on both screens | one simulation. This is the whole feature | two grants leaked, or `0x03D9` is not going out. `grep 0x03D2` in the log and count: it must be **one per mob**, not two |
| loot after a kill A did 90 % of and B finished | items on A's screen only | the audience walk fell through to the killer - the log line names which candidate won |
| A walks out, B stays | B's mobs start moving within about a tick | `release_all` is not on the `Drop`/leave path. `held_by` in the log line says so without a second launch |

**What a run cannot settle:** whether `0x0224 UserEnterField` actually puts a `CUser` in the
pool. `session/multiplayer.rs` records that as the one unverified hop, and everything here
sits on top of two players being able to see each other at all. If they cannot, none of the
above is measurable and the answer is one of the six gates in `research/user-enter-field.md`
§5.
