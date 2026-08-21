# The test plan for the next client run

**Written 2026-08-20.** `STATUS.md`'s START HERE carries the one-line summary of each step;
this is the procedure. If the two ever disagree, this file is the detailed one and
`STATUS.md` is the index — fix both.

**A run costs the owner a manual launch, so this is ordered to get the most out of one.** The
steps that can kill the client are deliberately **last**, so that a crash at step 9 still
leaves steps 1–8 measured. That is not the order the summary table is in; it is the order to
actually do them in.

---

## Before you launch

**One elevated PowerShell window.** The path is absolute because an elevated window opens in
`C:\Windows\System32`, not in the repo.

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

**`-SetFieldProbe` is not optional.** Its name is a fossil — it now means "the channel
answers at all". Without it `Session::handle` returns nothing for *every* packet and the
client sits on "Connecting…", which looks exactly like a server that is not running. That
cost a launch on 2026-08-20.

**Do not pass `-SessionTokens`.** The measurement it existed for is done and it produces a
"trouble connecting" dialog.

**If the UI freezes at any point, it is almost certainly an unanswered packet, not a crash.**
The client blocks its whole interface — including the quit prompt's OK button — waiting for a
reply. Open `world.log` and find the last inbound line with nothing after it. That is the
packet nobody answered.

### Numbers you will need

| | |
|---|---|
| map 40 | 40 spawn points, all **Blue Snail** (template 2) |
| alive at once, solo | **30** — spawn capacity is 75% of the points below 6 players |
| respawn | **7 s** per point. Every point on map 40 has `mobTime = 0`, which means "no node, use the field's ordinary rate" — *not* "never" |
| Blue Snail | **45 HP**, gives **2 EXP**, hits for **3** — read out of the client's own `Mob.wz` |
| level 1 → 2 | **15 EXP**, so **8 snails** at 1x and **4** at 2x |
| snail drops | mesos **100%** (2 of them), Snail Shell **40%**, Bronze Ore 6%, Garnet Ore 6%, Crossbow Arrows 5% (10–30) |
| a drop on the floor | disappears after **2 minutes** |
| a level | **+16 max HP, +12 max MP, +5 AP** (changed today — it was +14/+10) |

---

## Phase 1 — the field fills itself

### Step 1. `!map 40`, then stand still for ten seconds

**Expect:** an **empty map**, then snails appearing over roughly the first seven seconds,
up to about thirty of them.

**The empty arrival is correct, not a bug.** The owner set that model: *"on first enter, no mobs
should exist until the respawn timer kicks in."*

**If nothing ever appears:** the field seeded but never refilled. `world.log` will show
whether `0x03C6` mob-spawn packets went out at all.

### Step 2. Kill one snail, walk away, come back

**Expect:** the map still populated, and the snail you killed replaced within about seven
seconds.

**Why it is here:** mobs live on the **channel**, not on your connection. Walking away must
not reset the field. This is the cheapest check of that.

---

## Phase 2 — combat and the HP bar

### Step 3. Hit one snail exactly once and read the bar above it

**Expect:** the green bar drops by roughly *your damage ÷ 45*. If you hit for about 18, the
bar should sit near **three fifths**.

**If the bar drops to about a quarter instead**, the percentage fix did not land — the field
is being sent as an absolute again. `0x03F0` carries a **percentage**, and this is the single
most likely place for that regression to reappear.

**Note:** the server does not compute your damage; the client does and reports it. So the
number varies with your gear, and the *ratio* is what is being checked, not the value.

### Step 4. Let a snail hit you

**Expect:** your HP bar goes down, about 3 per hit.

**Why it is still worth one look:** this direction of damage was retracted once on a static
analysis and then disproved by you in ten minutes. It works; this is a regression check, not
a question.

---

## Phase 3 — drops and pick-up

### Step 5. Kill a snail and look at the floor

**Expect:**

* the drops land **where the snail died**, not at your feet;
* when more than one drops, they are **spread slightly apart**, not stacked on one pixel;
* every kill drops **2 mesos**, and about **two in five** also drop a Snail Shell.

**This has been wrong twice**, both times landing at the player. The second time the mob was
removed from the field before anything asked where it had been.

### Step 6. Walk over a drop

**Expect:** it enters your bag, and a line appears **bottom-right**.

**If you see raw yellow opcode text in the chat log instead**, the pick-up is not being
answered — but `0x032C` is measured now, so this should simply work.

### Step 7. Kill something, then wait two minutes without picking it up

**Expect:** the drop disappears from the floor.

**Do this one while doing something else** — start the timer, carry on with Phase 4, come
back. It is the slowest cheap check in the plan.

---

## Phase 4 — experience, messages, and the level

### Step 8. Watch the bottom-right corner as you kill

**Expect:** `You received EXP (+2)` in the **screen message area**, bottom right.

**Not in the chat log.** The chat notices were removed. And the wording differs from the live
server — this build's string table says "received", not "gained", so that is not a bug.

### Step 9. Kill eight snails and level up

**Expect:** the level-up animation, and in your stat window afterwards:

* **max HP up by 16**
* **max MP up by 12**
* **+5 AP** to spend

**+16 / +12 is new today.** It was +14 / +10, which was a placeholder. If the numbers come
out wrong, **this client cannot arbitrate** — it has no per-level HP/MP table and is simply
told the new maxima. The source is a measurement of the live COT2 service, so a disagreement
means the source is wrong, not that we read a listing wrong. See goal K in `STATUS.md`.

---

## Phase 5 — the quest counter

### Step 10. Accept Sam's Suggestion, then kill snails

**Expect:** the quest counter moving, `1/10`, `2/10`, up to ten.

**If it reads `0/10` forever:** the count is sent as a **string** — three zero-padded decimal
characters per slot — and an integer renders as nothing at all. That is where to look first.

---

## Phase 6 — rate events and the scrolling banner

**Nothing here can crash the client**, which is why it comes before the window tests.

**What the cycle actually is, since this was unclear:** the *rate* never expires. It stays
where you put it until you type `!exprate 1`. The **banner** is what cycles — up for two
minutes, down for three, up again at the five-minute mark, repeating for as long as either
rate is not 1x.

### Step 11. `!exprate 2`

**Expect:**

* a chat line confirming the change (that part is just an ack);
* **a banner across the top of the screen** reading
  `[Event] The Server's EXP rate has been set to 2x`.

**This is the first time `0x00AC` has ever been sent to this client.** If nothing draws,
**check `world.log` for `BroadcastMsg type 4` before concluding anything** — the packet going
out and nothing appearing is a completely different problem from the packet never going out,
and only the log tells them apart.

Also worth saying plainly: **that it scrolls, and that it sits at the top, is inferred, not
measured.** If it turns out to be a static bar or a popup, the packet is still right and only
the name is wrong.

### Step 12. Kill four snails

**Expect:** to **level in four kills instead of eight**, and `+4` per kill in the corner.

That is the EXP rate actually applying, rather than just being stored and announced.

### Step 13. `!mesorate 3`, then kill a snail

**Expect:**

* **one** banner carrying **both** sentences, EXP first then Meso;
* **6 mesos** from the kill instead of 2.

The client has one banner object, so two events share one line. Two banners is not a thing it
can do.

### Step 14. `!map 40` while the banner is up — **this is a question, not a check**

**Watch:** does the banner survive the map change?

**Either answer is worth having, and neither is a bug:**

* **It survives** → leave the code alone.
* **It vanishes** → `world::session::rates` needs to re-assert the banner on field entry.

Nothing re-asserts it today, deliberately: re-asserting would restart the scroll on *every*
map change, which is a visible wrong answer in the common case, traded against a silent one
nobody has measured. Your answer decides it.

### Step 15. Leave it alone for five minutes

**Expect:** the banner disappears about two minutes after it went up, then comes back about
three minutes after that.

**Start this and go do Phase 7 while it runs.**

### Step 16. `!exprate 1` then `!mesorate 1`

**Expect:** after the first command the banner stays up carrying only the Meso sentence;
after the second it **comes down immediately**.

`world.log` should show a two-byte body for the teardown — type 4 with the flag at 0 and no
string at all.

---

## Phase 7 — the window teardown crash

**Everything from here can end the session. Do it last, and do it in this order** — the point
is to find out *how much* crashes, so stop at the first one that does.

### Step 17. Open the **inventory** window, then close it

### Step 18. Open the **stat** window, then close it

**Expect (hopefully):** both survive.

**What each outcome means:**

| what happens | conclusion |
|---|---|
| both survive | the teardown is **specific to the skill window**, and the crash is worth chasing |
| either one crashes | the teardown is **generic**, skills are innocent, and the earlier diagnosis was looking at the wrong thing |

**This step costs no code and answers a question that has been open since yesterday.** It is
the highest-value thing in the plan.

### Step 19. Open the skill window and click `+` on Three Snails — **twice**

**Expect:** the level going up on both clicks.

**Click it twice on purpose.** The first click always went out; the second was swallowed by a
latch that only a server packet clears. One click proves nothing.

### Step 20. Close the skill window

**This is the step that has crashed the client.** If it does, the run is over and that is an
acceptable ending — everything above is already measured.

---

## Phase 8 — persistence

### Step 21. If the client is still alive, relog

**Expect all of these to survive:**

* your level, max HP and max MP
* your EXP total
* the quest counter
* the Three Snails skill level
* the items you picked up
* **the rate and the banner** — the rates live in the database, so a relog rejoins the event
  already in progress

**Do not click Log Out and then keep testing.** An unanswered Log Out poisons every later
`SetField` in silence. Relog properly.

### Step 22. Do not click Lucy

The shop is off by default and they will simply talk. There is no point spending attention on
them: we now know we have been sending the **wrong shop opcode** — `0x0560` opens the window
whose art this client does not have, and `0x055D` opens the one it does — but the `0x055D`
body is not decoded yet, so nothing has changed. `research/classic-shop-opcode.md`.

---

## After the run

**The output is the most expensive thing this produces. Do not lose it.**

`tools/test-server.ps1` moves the previous run into `previous-runs/` instead of deleting it,
but that is a rolling buffer. **Copy anything that settled a question into
`research/fixtures/` under a name that says what it proves.**

| file | what is in it |
|---|---|
| `world.log` | every packet both ways on channel 0 — read this for anything past character select |
| `world-ch1.log` | channel 1 |
| `login.log` | the login connection |
| `client-patched\maplecw-hook.log` | `WATCH` lines, session patches, client faults |
| `client-exit.log` | how the client died. A clean `0` is a hand-close |

**And one free measurement nobody has ever taken.** Every `-SetFieldProbe` run dumps the
client's own EXP curve on the positive control's first hit:

```bash
python "C:\MapleCW\tools\decode_dump.py" --exp-curve
```

Compare it against `data/exp-curve.txt`. It is a five-second job. **If they disagree, the
client wins** — it draws the EXP bar from its own copy, so a server that disagrees shows a bar
that does not fill when it should.

---

## Two watches that need their own runs

Neither combines with the above; each costs a separate launch.

* **the user state machine** — `-SetFieldProbe -UserState`. What sets `user+0x5e4` to 18/19,
  the predicate four of the six attack builders test and the one that refuses the pick-up
  pre-check.
* **mob targeting** — `-SetFieldProbe -MobTargets`. Kept, though the mob question is answered.
