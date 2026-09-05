# The mob's flinch and knockback: the attacker plays it locally, and only if it holds the grant

Written 2026-09-04 in answer to the owner: *"If the client that does not have mob control attacks a
mob, the mob does not flinch and get pushed back, this is not okay and the behavior should
mimic the client that does have mob control."*

**No Ghidra** (the project lock is held elsewhere), **no client run** — this is `tools/*.py`
over `client-patched\MapleStory.exe` plus two archived runs that between them contain the
experiment nobody realised had already been performed.

Tags as everywhere in `research/`: **[L]** read off this client's listing or off a capture,
**[D]** derived from two or more [L], **[I]** inferred.

---

## 0. The answer, and it is settled

**The hit reaction is produced locally by the client that both swings *and* holds the mob's
`0x03D2` grant. It reaches every other screen as that client's own `0x02FF` mob-move report
with action `7` = `hit1`, which this server already rebroadcasts as `0x03D9`. No packet the
server sends can produce it, and there is no `MobDamaged` opcode in this client.**

Two archived runs settle it. Same build, same map (40), same two characters, same mobs, and the
one thing that differs is **which client walked in first and therefore owns every mob on the
field**:

| run | controller | who swung | landed hits | of which wounded | `0x02FF` reports | with action 7 |
|---|---|---|---|---|---|---|
| `previous-runs/world-20260903-225142.log` | **Tester2 (214)**, first in at 02:44:58 | **Tester2 (214)** | 15 | 10 | 10 990 | **10** |
| `previous-runs/world-20260904-190343.log` | **Cobalt (213)**, first in at 23:01:15 | **Tester2 (214)** | 22 | 15 | 3 936 | **0** |

> **Both files are in `previous-runs/`, which is a rolling buffer.** They are the only evidence
> for everything below and neither has a copy in `research/fixtures/` yet. Copy them before they
> roll — `CLAUDE.md` § *A run's output is evidence*.
>
> The second file was `world.log` while this was being written; the launcher rotated it at
> 19:26 when the next run started. Same 4 374 564 bytes, same content.

In the first run the pairing is exact: **fifteen landed hits, five of them killing blows, ten
flinches — one per wounding hit, 0.20–0.45 s after it, and none for a killing blow** (a kill
draws `die1`, not `hit1`). In the second run there were **fifteen wounding hits and seven kills**
— the same shape of fight, more of it — and the flinch count is **zero** out of 3 936 reports.
**[L]**

The wounding/killing split is read off the log rather than assumed: `0x03F0` is logged twice per
wounding hit (the attacker's copy and the map-wide publish) and `0x03D1` twice per kill, so
30/2 + 14/2 = 15 + 7 = 22, which is exactly the `0x00DF` count. The first run is 20/2 + 10/2 =
10 + 5 = 15, plus four swings that hit nothing, which is exactly its 19.

And the whole chain is visible on the wire in the first run, end to end:

```text
02:48:54.107  ->  0x03F0  mob 2002 took 14 (45 -> 31)          the server's bar update
02:48:54.121  ->  0x029E  character 214 hit 1 mob(s) for 14    the remote-attack broadcast
02:48:54.534  <-  0x02FF  mob 2002, action byte 0x0f           <- 214's OWN client flinches it
02:48:54.558  ->  0x03D9  mob 2002, action byte 0x0f           <- relayed to the other client
                          "140 path bytes copied verbatim" = 14 + 6*21, six elements.
                          The four walk reports before it on that mob are 56, 35, 77 and
                          35 - one to three elements. The second flinch, 02:48:55.539, is
                          119 = 14 + 5*21. The recoil is IN the flinch report's path.
```

Ten for ten, in both directions. **[L]** The knockback is not a separate thing: it is the
six-element path inside that same flinch report (a walk report on that mob is 35–77 path bytes;
the flinch is 140), and `0x03D9` carries it verbatim.

So:

* **the fix is server-side**, and it is the branch `Controllers::hand_over` already exists for:
  **give the attacker control of the mob it is hitting.** §5.
* **no new packet body is involved.** The `0x03D2` this needs is byte-identical to the one
  `Session::hand_over_mobs` already sends, and that packet was delivered thirty times in one
  burst in the 09-03 run and demonstrably worked — §5.2 is that measurement, and it also
  retires the one hazard that could have killed this approach.
* `0x03F0` and `0x029E` are **eliminated**, each with its own reading rather than by absence.
  §3.

---

## 1. The instruments, and the control each one passed first

`CLAUDE.md` § *Verify the instrument before believing it*. Everything was run **from the repo
root**; the three throwaway scripts were piped in as `python - args < script.py` so `sys.path[0]`
is empty (§ *The scratchpad shadows the real tools*).

| instrument | control | result |
|---|---|---|
| `tools/reads.py` | `0x140304100 2` → raw@`140304138`, u8@`140304144`, u8@`140304183`, then a `u16` run | reproduced exactly |
| `tools/listing.py` | same function, same reads at the same addresses | reproduced |
| `tools/callers.py` | `0x1402fa9a0` → 96 sites in 15 functions, 43 in `0x140304b20` | reproduced |
| `tools/rangescan.py` | `+0x2f4` over the mob range → `141cb7ef3 MOV [R12+0x2f4],EAX` | reproduced |
| archive action histogram (§2.1) | must find action 7 somewhere, or a zero anywhere means nothing | **318 events found**, spread over 40 runs |
| forward-reachability scan (§4.2) | `FUN_141c813b0` must reach `FUN_141c56870` at depth 1 — `callers.py` lists it | reproduced |

### 1.1 Two of my own instruments were wrong first, and both would have answered confidently

Recorded because the file that will be read next is this one, not my scratchpad.

**(a) A `jmp` is not a call.** My first reachability scan counted every `jmp rel32` as an edge
and reported that the melee attack builder `FUN_1428c1fa0` reaches `CMob::SetMoveAction` at
depth 4. The path it printed was:

```text
1428c2c2d  in FUN_1428c1fa0  -> FUN_141d31b20     a real call, the mob collector
141d31bab  in FUN_141d31b20  -> "FUN_141d31bb1"   <- NOT a call. jmp +6, INSIDE the same
                                                     function: .pdata says 0x141d31b20 ..
                                                     0x141d32b1c, and 141d31bb1 is +0x91
```

Counting only `call rel32` plus a `jmp rel32` whose target is **outside the jumping function's
own merged `.pdata` extent**, the answer inverts: the four `0x00DF`/`0x00E0`/`0x00E1` attack
builders reach `SetMoveAction` **not at all, to depth 5**. That is §4.2's negative, and the
first version of it was a false positive that pointed at exactly the conclusion I wanted.

**(b) `tools/mobpool_tables.py` silently mislabels two opcodes.** It decodes each dispatch stub
by taking the first `call`/`jmp` with an absolute target. Two stubs make a **virtual** call and
have no absolute target, so the tool walks past it to the stub's trailing `jmp 0x141d33432` and
reports the shared epilogue as the handler — printed as `handler FUN_141d33432 (0 bytes)`, which
reads as "not handled":

```text
141d331b2  mov rax,[rbx] / mov rdx,rdi / mov rcx,rbx / call qword ptr [rax + 0x118]   0x041D
141d331c6  mov rax,[rbx] / mov rdx,rdi / mov rcx,rbx / call qword ptr [rax + 0x120]   0x041E
```

**[L]** `0x041D` and `0x041E` are dispatched through mob vtable slots 35 and 36. They are
handled, and §4.1 names what they do. The tool's own header says every live stub is
`MOV RDX,RDI / MOV RCX,RBX / CALL handler`; that is true of 100 of the 102, and the two
exceptions are the two it gets wrong. **Anyone reasoning about "which mob opcodes this client
ignores" from that tool's output is reading two false negatives.**

---

## 2. The measurement

### 2.1 Archive-wide: 318 flinches in 181 469 mob-move reports, and where they are

De-duplicated on `(timestamp, opcode, body hex)` across `previous-runs/` and
`research/fixtures/` — 542 files — because those two directories overlap and a fixture copied
mid-write hashes differently from its own run (`CLAUDE.md`, and `research/mob-attack-skills.md`
§1.1 for the eleven such pairs). **[L]**

```text
distinct 0x02FF events   181 469
  0xff   181 151   action -1, idle
  0x0f       201   action 7 facing 1   hit1
  0x0e       117   action 7 facing 0   hit1
  attack range 0x1a..0x3b : 0        skill range 0x3c..0x5d : 0
```

The split is `action * 2 + facing` when `(unsigned)action <= 0x55` and the raw byte otherwise —
`1402b3b30 CMP ECX,0x55 / SETBE AL / RET`, reached at `141cb7e88` in the move builder, with the
packing at `141cb7e97` on the true side and the raw store at `141cb7ea1` on the false side.
That is `research/mob-attack-skills.md` §1.2's reading and this is an independent re-run of it
against a larger archive; the ratios and the absence of the attack/skill ranges hold.

**Of the 40 runs that contain a flinch, exactly one is a two-client run** — that is, exactly
one ever sent a `0x0224 UserEnterField`, which `Bus::enter_field` posts only when a second
connection exists. That run is `world-20260903-225142.log`, and it is the whole experiment.

### 2.2 `world-20260903-225142.log`: the attacker was the controller, and it flinched

```text
02:44:58.863  0x01A0 SET_FIELD   character 214 (Tester2)     <- FIRST IN
02:45:06.355  0x03D2 x30         "to this client, which claimed it"   <- 214 owns every mob
02:45:20.349  0x01A0 SET_FIELD   character 213 (Cobalt)       <- second
02:48:39..41  0x029E             character 213 hit 0 mob(s)   <- 213's four swings hit NOTHING
02:48:54..    0x029E x15         character 214 hit 1 mob(s)   <- every landed hit is 214's
02:51:00.913  "mob control: 30 mob(s) on map 40 handed from connection 1 to connection 3"
```

Ten flinches, all on mobs 214 was hitting, each 0.20–0.45 s after its `0x029E`:

```text
02:48:54.534  mob 2002  0x0f     02:49:01.925  mob 2001  0x0f
02:48:55.464  mob 2002  0x0f     02:49:02.735  mob 2001  0x0f
02:48:58.054  mob 2004  0x0f     02:49:07.346  mob 2002  0x0e
02:48:58.955  mob 2004  0x0f     02:49:08.127  mob 2002  0x0e
02:50:40.334  mob 2002  0x0e     02:50:41.064  mob 2002  0x0e
```

Per mob: three hits, two flinches, one death. **Every wounding hit, and only a wounding hit.**

**213's four swings are not a counter-example and must not be used as one.** They hit zero
targets — `0x029E ... hit 0 mob(s) for 0, 61 bytes` — so they say nothing about what a
non-controller's *landed* hit does. That is the same trap `CLAUDE.md` records under *"the thing
you are comparing against may never have been a control"*, and it is why the second run matters.

### 2.3 `world-20260904-190343.log`: the attacker was not the controller, and nothing flinched

```text
23:01:15.596  0x01A0 SET_FIELD   character 213 (Cobalt)      <- FIRST IN
23:01:23.091  0x03D2 x30         "to this client, which claimed it"   <- 213 owns every mob
23:01:35.998  0x01A0 SET_FIELD   character 214 (Tester2)      <- second
23:01:36.576  0x0224             UserEnterField: Tester2
every 0x0293 and every 0x029E in the file names character 214             <- 213 never swung
```

22 swings, 22 `0x029E`, 30 `0x03F0` (15 wounding hits), 14 `0x03D1` (7 kills), 3 516 `0x03D9`,
and **3 936 `0x02FF` reports of which every single one carries `0xff`** — action −1, idle. Not
one flinch, on any client, for the whole run. **[L]**

The controller (213) received all 22 `0x029E` (`Bus::publish` excludes only the sender) and all
30 `0x03F0`, and its mob simulation did nothing with either.

### 2.4 What the two runs jointly rule in and out

| candidate | verdict |
|---|---|
| the attacker's own swing, unconditionally | **out** — 15 wounding hits in 2.3, no flinch |
| the controller receiving `0x029E` | **out** — 22 delivered to the controller in 2.3, no flinch |
| the controller receiving `0x03F0` | **out** — 30 delivered in 2.3, no flinch, and §3.1 reads the handler |
| **the attacker's own swing on a mob it controls** | **in** — 10 for 10 in 2.2, 0 for 22 in 2.3 |

**The named blind spot.** These two runs differ in who owned the mobs *and* nothing else that
was measured, but they are two runs, not one run with a switch flipped. A confounder that
tracks "which client logged in first" would produce the same table. Nothing in the logs suggests
one; it is still the honest weakness of the comparison, and §7's first check retires it in the
same launch that tests the fix.

---

## 3. The two packets that were already going out, eliminated one at a time

### 3.1 `0x03F0 MOB_HP_CHANGE` cannot flinch anything — the whole handler, 241 bytes

`FUN_141c83440`, read end to end rather than sampled. **[L]**

```text
141c83458  READ u32  ->  mob+0x8b4 AND mob+0x8bc          the HP
141c8346c  READ u8   ->  setne sil                        showBar
141c8347c..141c834a7   five template gates
141c834c2  jmp 0x141cd7b40                                the boss-gauge redraw, tail call
   otherwise
141c834d1/e9  push the number into mob+0x6d8              the floating display list
141c8351b  mob+0x6c0 = now                                its timestamp
141c83530  ret
```

`tools/reads.py 0x141c83440 2` reports **exactly those two reads and no more**, so there is no
gated tail. The function never touches `mob+0x3e8` (the action), never touches the position, and
the forward-reachability scan of §4.2 says it cannot reach `CMob::SetMoveAction` at depth 5.
**The health bar is all this packet does, and that is all it was ever claimed to do**
(`research/mob-hp-bar.md`, `research/mob-combat.md` §12).

### 3.2 `0x029E..0x02A1` does not drive it either — and here the negative needs its blind spot named

`FUN_1429d2ee0`'s readable 737 bytes make **13 distinct calls, enumerated, and not one of them
is in the mob code range `0x141c40000..0x141d60000`**:

```text
1406e8b80  1407b2910  14079f5e0  140c934d0  140c93640 x2  140f32200  140f32440
1415f0df0  1429e3ef0  142e541f0 x2  142ef4450  142ef44fc  142ef8250
141d2efc0 (the mob-pool lookup)  -  ABSENT
141c56870 (SetMoveAction)        -  ABSENT, and unreachable at depth 5
```

It decodes the header (`FUN_140f32200`) and the target list (`FUN_140f32440`) into a stack
struct and then `1429d31bc jmp 0x144dad6a6`, which is inside `.themida` — raw size 0, no file
bytes. **[L]**

**The blind spot, stated as `research/remote-attack-verification.md` §6.1 already stated it:
nothing static can rule out a mob touch inside the VM.** What rules it out here is not the scan,
it is §2.3: twenty-two of these were delivered to the mob's controller and its simulation did
nothing. That is a measurement, and it is the reason this row is "out" rather than "unknown".

---

## 4. What *can* set a mob's action, enumerated rather than searched

### 4.1 Every mob inbound opcode, both dispatch tables

The mob pool has two: `FUN_141d30e80`'s 19-entry table at `0x141d31184` covering
`0x03C6..0x03D8`, and `FUN_141d32b30`'s 117-entry table at `0x141d33448` covering
`0x03D9..0x044D` — `research/mob-behaviour.md` §2 and §2.2. All 121 slots were enumerated, the
two vtable-dispatched stubs of §1.1(b) included, and each handler tested for reachability to
`CMob::SetMoveAction` = `FUN_141c56870`. **[L]**

The complete set that can change a mob's move action:

| opcode | handler | what it sets | takes damage? |
|---|---|---|---|
| **`0x03D9`** | `FUN_141c813b0` | the action **from the packet**, body offset 5, plus position from the path | no |
| `0x03E8` | `FUN_141c82390` | `[rsi+1]`, from an internal value | no |
| `0x03FA` | `FUN_141cc2b70` | a **timed override**: `mob+0x980 = now + u32`, `mob+0x984 = u32 action` | no |
| `0x0400` | `FUN_141d3a7a0` | the action **from the packet** (`141d3a7b6` u32 → `141d3a7d3`), then a script/COM tail | no |
| `0x0410` | `FUN_141ce0940` | hard-coded `74` (`skillUse`), gated on the current action being 30 or 51 | no |
| `0x041D` | vtable slot 35 = `FUN_140ff8050` | hard-coded `49` (`miss`), gated on `u8 type == 4` | no |

**There is no `MobDamaged`.** The two tables have 19 + 117 = 136 slots, 98 distinct handler
addresses once the shared default stub and the duplicates are collapsed, and **not one of them
reads a damage value and reacts to it.**
The third reading in the brief — *"a separate packet exists that this server has never sent"* —
is answered: the only action-and-position setter in the set is **`0x03D9`, which this server
already sends 3 516 times in a two-minute run**.

Two of these were worth reading anyway and are recorded so nobody re-derives them:

* **`mob+0x980` / `mob+0x984` is a timed action override**, and `0x03FA` is its only writer in
  the whole mob range (`tools/rangescan.py` gives 8 sites for `+0x980` and 6 for `+0x984`, all
  accounted for). `CMob::Update` applies it at `141c62950`: while `now <= mob+0x980` and the mob
  is idle, its action becomes `mob+0x984`; past the deadline a single `mov qword [r12+0x980],0`
  clears both. `FUN_141cc2810` is an out-of-line copy of the same block with zero callers.
  It is the right *shape* for a flinch and it is not wired to damage — `0x03FA` also reads a
  string and does UI work, so it is not a candidate to fire on a hit.
* **`0x041D` type 4 is the mob's dodge**: `SetMoveAction(mob, 49 = miss)`, then it rewrites the
  packed animation triple at `mob+0x3dc/0x3e0/0x3e4` with the facing bit from the packet, redraws
  through `[vtable+0xc0]`, and plays a sound through `FUN_1429f28f0(.., 0x4c, 0x64, ..)`. Any
  other `type` byte returns having read one byte. Recorded because it is the closest thing in
  the client to a server-driven combat reaction, and it is a *miss*, not a hit.

### 4.2 Nothing in the local attack path sets it either — which is why control is the gate

Forward reachability by real calls only (§1.1(a)), depth 5, control passing:

```text
control  FUN_141c813b0 (0x03D9)      -> FUN_141c56870 at depth 1     PASS
         FUN_1428c1fa0  0x00DF melee  ->  NO
         FUN_1428cb4a0                ->  NO
         FUN_1428cd6d0  0x00E1 magic  ->  NO
         FUN_1429a4eb0                ->  NO
         FUN_1428bc4d0                ->  NO
         FUN_141c83440  0x03F0        ->  NO
         FUN_1429d2ee0  0x029E        ->  NO
         FUN_141c626c0  CMob::Update  ->  depth 1
```

`CMob::Update` is the only non-packet route, and the mob's own AI runs against the obfuscated
controller state at `mob+0x2e4`, whose only server-driven writer is the `0x03D2` grant's
vtable-slot-8 call (`research/mob-behaviour.md` §4, `research/mob-attack-skills.md` §3.1).

**Named blind spot, and it is the reason §0 leans on the runs rather than on this table.** The
attack builders make many `call qword ptr [rax + N]` through the mob's own vtable, and a
register-indirect call leaves no trace in any of these three scans. So §4.2 is *"no direct call
path"*, not *"the swing never touches the action"*. What makes the conclusion safe is that it
agrees with the two runs, which measure the whole client including its vtables.

### 4.3 The action really does come from the caller, not from the mob

`FUN_141cb6880` — the move builder, CMob vtable slot 22 at `+0xb0`, the thing that writes an
outbound `0x02FF` — takes the action as **argument 2**: `141cb68b9 mov [rbp-0x70], edx`, read
back at `141cb7e83` and packed at `141cb7e88`/`141cb7e97` into body offset 7. It does **not**
read `deobf(mob+0x3e8)` for that byte. **[L]** So a mob that is merely *in* `hit1` does not
report `hit1`; something has to call slot 22 with `7`. That is a property of the client we do
not control, and it is another reason a server-side "make it flinch" packet is not obviously
available: setting the action is not the same as reporting it.

---

## 5. The change: hand the mob to the attacker

### 5.1 What to do

In `crates/world/src/session/combat.rs::on_attack`, inside the target loop, for each target the
session does **not** already control:

1. re-assign the mob to this session in `Fields::controllers()` — this needs a
   `Controllers::hand_over_one(map, object_id, to)`, the single-mob twin of the existing
   `hand_over(map, from, to)`, taken under the same one lock so there is no instant with two
   owners and none with zero;
2. push one `0x03D2` **into `out`** — the attacker is `self`, so no bus hop is needed;
3. build it exactly as `Session::hand_over_mobs` already does:
   `net::mobmove::mob_change_controller(&mob.as_seen(), net::mobmove::CONTROL_NORMAL)`.

**`as_seen`, not `spawn`, and `CONTROL_NORMAL` (= 1), not 0.** `as_seen` hands the mob over at
its current position and HP, so the new controller resumes the wander where the monster is
standing instead of teleporting it back to its spawn point.

**On level `0`:** it RELEASES, and `crates/net/src/mobmove.rs::CONTROL_RELEASE` is the one
place that says so. This file said *"it despawns - straight into the pool's erase path"* until
2026-09-04, which was a restatement of a claim that had lost its two guards on the way here.
A release to the old holder is now half of every handover. Level `1` rather than
`2` because `> 1` is the aggro flag and takes the branch with the unguarded `mob+0x2c0`
dereference (`research/mob-behaviour.md` §4.1).

**Nothing goes to the old controller.** There is no non-destructive revoke in this client. Its
client keeps simulating locally until its next `0x02FF` for that mob is refused by
`may_report_movement` and goes unacknowledged, at which point it runs one step and stops
(`research/mob-behaviour.md` §10.1) — and from then on the new controller's `0x03D9` drives its
screen, overwriting position, animation and `mob+0xcd0` (`141c81784`), which is exactly what
`0x03D9` is for. **That cost is measured, not guessed: the 09-03 run contains exactly two
`0x02FF for mob N IGNORED` lines in 10 990 reports** — one per control change, each of them the
old controller's last stale report for a mob it had just killed. One straggler per handover.

**Byte-exact field list: there is none to get wrong.** No new packet, no new field, no new unit.
The `0x03D2` body is the one `net::mobmove::mob_change_controller` already builds and this
server already shipped 30 times in a single burst at 02:51:00.976 on 2026-09-03.

### 5.2 The one hazard, and the archive already retired it

`0x03D2`'s handler calls mob vtable slot 8 `FUN_141c54200(mob, 1)` (`141d34ca9 MOV EDX,1`), and
that function contains a short-circuit that would have killed this whole approach:

```text
141c5420a  rdi = [rcx + 0x2c0]        the second animation object
141c5423a  rcx = rdi - 0x20
141c54241  call 0x1409c5080           -> FUN_1409d4840(anim + 0x108)
141c54246  test eax,eax
141c54248  jne  0x141c5436e           <-- NON-ZERO: jump to the EPILOGUE. State is not set
                                          to 3, FUN_141c55750 is not called, nothing happens
141c5424e  lea edx,[rax+3] / call 0x141c4ff30    state := 3    (the activation)
```

The same predicate is required to be **non-zero** for `CMob::Update` to do its work
(`141c6331f`, `141c6340e`), so it reads as *"this mob is live and animating"* — and a mob being
re-granted mid-fight is, by definition, live and animating on the receiving screen. On the
listing alone, **a handover to a client that can already see the mob looks like a no-op.**

It is not, and the proof cost no launch:

```text
02:51:00.913   mob control: 30 mob(s) on map 40 handed from connection 1 to connection 3,
               30 grant(s) delivered
02:51:00.976   0x0225 UserLeaveField: character 214 is gone     <- connection 1 is gone;
                                                                  anything after this is
                                                                  connection 3 and nothing else
02:51:02..42   EXACTLY 30 inbound 0x02FF per second, every second, one per mob, for 41
               seconds, until "ch0 #3 ... closed" at 02:51:42.272
               1140 x 0x03E4 acknowledged in that window
```

Connection 3 had had all thirty mobs drawn on its screen since 02:45:20 and had never
controlled one. After the grant it drove every one of them. **[L] A `0x03D2` re-grant to a
client that already has the mob activates it.**

That is also the first on-the-wire confirmation of the departure handover itself, which
`STATUS.md` still lists as *"Not observed on a screen"*. It works.

### 5.3 What this costs, honestly

* **One `0x03D2` per mob per change of attacker.** Two players trading blows on one monster
  hand it back and forth once per swing. The body is a full mob record; on a 30-mob map with two
  players fighting the same mob that is one such packet a second, against the 30 `0x02FF` a
  second the field already carries. Cheap, but not free, and it should be gated on
  `controller_of(map, id) != self` so a controller hitting its own mob sends nothing.
* **The first hit of a fight still will not flinch.** The grant leaves with the reply to the
  swing that triggered it, so it lands after that swing has already been drawn. From the second
  hit on it works. In the 09-03 data a mob takes three hits and flinches twice, so this trades
  "0 of 2" for "1 of 2" on the first monster and "2 of 2" on every subsequent one, since the
  attacker keeps the grant. **[D]** If that is not good enough, the alternative is to hand the
  mob over on proximity rather than on the hit, which needs `0x00D9` parsed (nothing parses it
  today) and is a bigger change.
* **A stalled attacker freezes its mobs for everybody.** Already true of the first-arrival rule
  (`research/mob-share.md` §8 item 2); this concentrates control on whoever is fighting, which
  is the person most likely to be at their keyboard.

### 5.4 The alternative I am not recommending, and why

A synthetic `0x03D9` with the action byte forced to `0x0E`/`0x0F` would put the flinch on every
screen including the controller's, and `net::mobmove::mob_move_broadcast` already builds that
packet. It is tempting because the *only* difference from a packet shipped 3 516 times in one
run would be one byte.

It is not recommended, and the reason is the path: a `0x03D9` carries a movement path, and a
flinch's path is six elements of recoil that the server would have to author. `MobMoveRequest`
carries the last real path we saw, and re-emitting it would replay a walk; emitting a zero-element
path runs into `1404b26a9 MOVSX ECX,AX / TEST / JLE`, whose behaviour past that branch nobody
here has read. **A wrong length on this wire has killed this client twice** — `CLAUDE.md` says
so about the chat packet and the `0x02A5` echo — and the handover needs no new bytes at all.

---

## 6. What is NOT settled

* **Where in the client the flinch is decided.** I could not find the code that calls slot 22
  with action `7`. Its 43 `SetMoveAction` call sites were enumerated with the immediate each one
  passes in `edx`; the immediates that appear are 5, 45, 49, 51, 57, 73, 74, 75, 76 and
  `[rax+9]`, `[rdi+0x30]`, `[rbx+0x4c]`, `[rsi+1]` — **never 7**, and **24 of the 43** pass a
  register whose provenance a linear scan cannot follow. So §0's mechanism is *what the client does*,
  measured; it is not *where the client does it*. That gap does not affect the fix, and it does
  mean nobody should claim a gate for the flinch that has not been read.
* **Whether the first run's contrast is the ownership or something else about that session.**
  §2.4's blind spot. §7's first check settles it in the same launch as the fix.
* **`FUN_1409d4840`** — the predicate §5.2 works around. Its *effect* is measured; its meaning
  is not read.
* **Whether the non-controller's screen currently shows the damage numbers** from `0x029E`.
  Unrelated to this file's question, still open, and still behind the Themida jump.

---

## 7. What the owner should see, and what each outcome means

Two clients, one map with mobs. **One variant.** The first check costs nothing and should be
done first, at ~40 s of client life, because it separates "this thing" from "this session"
(`CLAUDE.md`):

**Check 0 — free, before the fix is even built.** Log in **A first**, let it claim the mobs,
then log in **B**. Have **A** — the one that owns the mobs — hit a snail twice without killing
it.

* the snail flinches and slides back on both screens → §0 holds, and §2.4's blind spot is gone.
  `world.log` says the same thing without anyone squinting: `grep 0x02FF` and look for a body
  whose 8th byte is `0e` or `0f`.
* it does not → the 09-03 run's ten flinches were something else, this whole file's mechanism is
  wrong, and nothing should be built on it.

**Then, with the handover wired:** same setup, but **B** — the one that owns nothing — attacks.

| watch | if it holds | if it does not |
|---|---|---|
| the mob on **B's own screen**, from the **second** hit onward | it flinches and is pushed back, exactly as it does for A | count the event in two logs: `world.log` must show a `0x03D2` for that object id right after the first hit, then `0x02FF` **from B** for it. If the `0x03D2` is there and no `0x02FF` follows, the grant did not activate — §5.2's predicate, and that is a different bug from "the flinch does not fire" |
| the same mob on **A's screen** | the same flinch, one `0x03D9` behind — 20–100 ms in the 09-03 data | the rebroadcast is not going out; `grep 0x03D9` for that object id and check byte 6 of the body |
| the **first** hit | expected to be flat. §5.3 | if the first hit flinches too, something else is also firing and it should be found before it is relied on |
| the mob's position on both screens while B fights it | one simulation, no jitter | A is still simulating locally: look for `0x02FF for mob N IGNORED` lines naming A's connection long after the handover |
