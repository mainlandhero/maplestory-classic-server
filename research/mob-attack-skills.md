# Why no mob has ever used an attack skill

2026-08-28. Static plus the archived logs — **no client run was spent on this.**
Tags: **[L]** read off the image or off a capture, **[D]** derived from two or more [L],
**[I]** inference.

Listings dumped beside this file: none — everything here came from `tools/listing.py`,
`tools/callers.py`, `tools/rangescan.py`, `tools/dataref.py` and `tools/xref.py`, run with
the repo as the working directory.

---

## 0. The answer in one sentence

**`CMob::Update` skips its entire attack-and-skill selection block unless the mob's
controller state is exactly `4`, the only server-driven writer of state `4` is the
`0x03E4 MobCtrlAck` handler when body offset 6 is non-zero, and
`crates/world/src/session/combat.rs:59` has passed a hard-coded `false` there since the
packet was first built.** **[L]**

```
141c63824  lea   rcx,[r12 + 0x2e4]          the mob's obfuscated controller state
141c6382c  mov   edx,[r12 + 0x2ec]          its key
141c63834  call  0x1401ba9d0                -> the plaintext state
141c63839  cmp   eax, 4
141c6383c  jne   0x141c63e7c                <-- NOT 4: skip 1 594 bytes, and that is
                                                the only attack selection in the client
```

Everything below is the working, the three independent measurements that agree with it,
and the one-line change with what to watch for.

---

## 1. What the archive actually says — and the premise in the brief was wrong

The brief said 198 mob-to-player hits and asked whether 198 samples is simply too few.
It is not a sample-size question, and the number is not 198.

### 1.1 De-duplicate by EVENT, not by file — `sha256` was not enough here **[L]**

`CLAUDE.md` says a glob over `previous-runs/` and `research/fixtures/` double-counts and
that hashing first fixes it. **Hashing the files did not fix it.**
`research/fixtures/skill-window-close-faults-world.log` and
`previous-runs/world-20260820-181822.log` are the same run; they differ at **char 74 of
line 1** and by **41 KB of tail** (the fixture copy was taken later), so they hash
differently and were counted twice. Eleven such pairs are visible in the per-file
breakdown, every one with an identical event count on both sides.

De-duplicating on `(timestamp, opcode, body hex)` is immune to that: two copies of one run
contribute identical tuples.

```
0x02FF mob-move reports    raw lines 243 418   distinct events 131 003
0x00E5 USER_HIT bodies     raw lines     435   distinct events     257
```

**The file-level count was very nearly double.** Any future count over those two
directories should use the event key, not the file hash.

### 1.2 Measurement A — 131 003 mob-move reports, zero attacks **[L]**

`0x02FF` body offset 7 is the mob's action. It is `action * 2 + facing` **when
`(unsigned)action <= 0x55`** and the raw action otherwise — that is not a guess, it is a
three-instruction function:

```
1402b3b30  cmp   ecx, 0x55
1402b3b33  setbe al
1402b3b36  ret
```

reached at `141cb7e88` in the move builder, with `141cb7e97 add bl,bl / or al,bl` on the
true side and `141cb7ea1 mov [rsp+0x5c],bl` (the raw action) on the false side. `-1` is
`0xFFFFFFFF` unsigned, so "no action" takes the raw side and lands as `0xFF`. That is why
every idle report is `0xFF` and never `0xFE`, which is the detail that settles the
otherwise ambiguous reading. **[L]**

Histogram of every distinct report ever archived:

```
  0x0e :        95   action 7, facing 0
  0x0f :       177   action 7, facing 1
  0xff :   130 731   action -1, no action
  ---------------------------------------------------------
  bytes in the attack range 0x1a..0x3b (actions 13..29) :  0
  bytes in the skill  range 0x3c..0x5d (actions 30..46) :  0
```

**Action 7 is `hit1`** — the flinch when the player hits the mob. See §2. So across every
run this project has ever archived, a mob has performed exactly **two** distinct actions:
nothing, and recoiling.

### 1.3 Measurement B — 257 user-hit reports, all contact **[L]**

```
0x00E5 USER_HIT   damageType {0: 257}   attackIndex {-1: 257}   damage {1: 257}
```

Same direction as the brief, corrected count. `-1` is the body/touch attack
(`crates/net/src/userhit.rs`).

### 1.4 Measurement C — the packet the attack block arms has never been sent **[L]**

`0x0313` has **exactly one builder in the whole image** —
`research/msexe-send-opcodes.txt` line 1641, `FUN_141c626c0 @ 141c63446` — and it is gated
on `mob+0xc8c != 0` at `141c63424`. `tools/rangescan.py 0xc8c` over the mob range returns
**three sites and no more**: that test, the clear at `141c63479`, and

```
141c63df1  mov  dword ptr [r12 + 0xc8c], 1     <-- inside the state-4 block
```

Enumerating every inbound opcode this client has ever sent gives **78 distinct opcodes**,
and `0x0313` is not one of them. Neither is `0x0300`.

*Positive control for that enumeration:* the same scan lists `0x02FF` and `0x00E5` with
large counts, so it is capable of seeing mob traffic.

*Blind spot, named:* `141c63de3 cmp [rdx+0x8c],0 / je` sits between the selection and the
`mob+0xc8c := 1`, so the absence of `0x0313` proves *"no attack with a non-zero `+0x8c`
was ever selected"*, which is a hair weaker than *"no attack was ever selected"*.
Measurement A does not have that gap.

**Three instruments, three different layers of the stack, one answer.**

---

## 2. The mob action index table, read out of the image **[L]**

This is the reusable artifact and it is why §1.2 is a measurement rather than a guess.
`FUN_141cc5b80(mob, idx)` indexes a table of animation names at **`0x143AAA670`**
(`141cc5be9 lea rcx,[rip+0x1de4a80]`, `141cc5bf0 mov rcx,[rcx + r14*8]`). The table is
**BSS — no file bytes**, so it cannot be read from the image; but `FUN_140009690` fills
it, one `mov rdx,[rip+A] / lea rcx,[rip+B] / call 0x1401a5890` per slot, with `B` walking
the table eight bytes at a time and `A` a `.data` slot that *does* have file bytes.

86 slots:

```
 0 move        1 stand       2 jump        3 fly         4 rope        5 regen
 6 bomb        7 hit1        8 hit2        9 hitF       10 die1       11 die2
12 dieF       13 attack1    14 attack2    15 attack3    16 attack4    17 attack5
18 attack6    19 attack7    20 attack8    21 attack9    22 attack10   23 attack11
24 attack12   25 attack13   26 attack14   27 attack15   28 attack16   29 attackF
30 skill1     31 skill2     32 skill3     33 skill4     34 skill5     35 skill6
36 skill7     37 skill8     38 skill9     39 skill10    40 skill11    41 skill12
42 skill13    43 skill14    44 skill15    45 skill16    46 skillF     47 chase
48 miss       49 say        50 eye        51 skillAfter1 .. 66 skillAfter16
67 sleep      68 wakeup     69 patrolSense              70 patrolUserdetect
71 patrolAttractdetect      72 patrolAttractarrive      73 transform
74 skillUse   75 skillFail  76 revive     77 sealed     78 runaway
79 directionAct1            80 remove     81 flip       82 groggy     83 heal
84 forceChange              85 forceChangeAfter
```

**Positive control, and it is exact.** `FUN_141cc5b80` accepts `idx - 0x33 <= 0xf`, i.e.
**51..66 and nothing else**, and its two call sites pass `deobf(mob+0x3f4) + 0x15` and
`[skill+0x28] + 0x33`. The table says 51..66 is `skillAfter1..skillAfter16` — sixteen
entries, exactly the accepted window, and exactly the thing you would look up after a
skill. A wrong base would not produce that.

It also makes the three range tests in the client readable at a glance:

| test | at | means |
|---|---|---|
| `lea eax,[r15-0xd] / cmp eax,0x10 / ja` | `141cb6a0a` | actions 13..29 = `attack1..attackF` |
| `add edx,-0xd / cmp edx,0x10 / ja` | `141c63ce3` | the same, in the selector |
| `add eax,-0x1e / cmp eax,0x10 / ja` | `141c63e0c` | actions 30..46 = `skill1..skillF` |
| `add eax,-7 / cmp eax,2 / jbe` | `141cb6b07`, `141c63355` | actions 7..9 = `hit1`, `hit2`, `hitF` |

---

## 3. The gate: controller state 4

### 3.1 The state field is a request/response cycle **[L]**

`research/mob-behaviour.md` §10.3 already established the field — `mob+0x2e4`/`+0x2e8`/
`+0x2ec`, three-word obfuscated — and its writers. What it did not ask is **what the
difference between 3 and 4 buys.** This does.

| who | writes | at |
|---|---|---|
| `0x03D2` grant, slot 8 `FUN_141c54200` | `3` | `141c5424e LEA EDX,[RAX+3]` |
| the move sender, immediately before `SendPacket` | `1` or `2` | `141cb8743` |
| **`0x03E4 MobCtrlAck`** | `2` if `ack < current`, else **`3`, or `4` if body offset 6 is set** | `141c8221d TEST R13D,R13D / SETNE BL / ADD EBX,3` |
| `CMob::Update` itself | `4`, under four narrow conditions — §3.3 | `141c63494`, `141c634ad`, `141c634d8`, `141c637e6` |

So state `4` exists **only in the window between an ack that granted it and the mob's next
move report**, which is precisely "the next action may be an attack". **[D]**

### 3.2 State 4 dominates the entire selection **[L]**

`141c6383c jne 141c63e7c` skips `0x141c63842 .. 0x141c63e7c`. Inside that block, and
nowhere else in this 20 427-byte function:

```
141c6395d  call FUN_141ca7240(mob, &action, &.., &..)   pick a SKILL   (writes mob+0x9ac,
                                                        reads/writes mob+0x2f8)
141c639b7  call FUN_141c7c900(mob, 5, &action, ...)     pick an ATTACK (17 702 bytes; the
                                                        AI proper)
141c63ce3  action-0xd, <=0x10  ->  141c63cf7 GetAttack(template, action-13)
141c63e0c  action-0x1e,<=0x10  ->  141c63e2a GetSkill(template, mob+0x2f8, mob+0x2fc, 0)
141c63df1  mob+0xc8c := 1                               arms the 0x0313 send
```

**Nothing branches into the block from outside.** Enumerating every branch and call in the
whole function with an immediate target inside `[141c63842, 141c63e7c)`: **28 of them, all
28 from inside the block, 0 from outside.** The function contains **no indirect jump at
all**, so there is no switch table to hide an entry. **[L]**

*Blind spot, named — two of them, and the second is the one that matters.*

1. The branch scan only sees direct control flow **within this one function's listing**. A
   `call` into the middle of the block from another function would not be matched — but
   there is no `.pdata` entry inside the range, and `tools/callers.py` is the tool that
   would find one.
2. **`FUN_141c7c900` is not exclusive to this block.** `tools/callers.py` gives it **13
   call sites in 9 functions**, and it contains a `GetAttack` of its own at `141c7d356`.
   So the sentence "state 4 dominates every attack selection in the client" is **not**
   established; what is established is *"state 4 dominates every attack selection in
   `CMob::Update`, which is the only place a chosen action reaches `FUN_141c551f0` and
   then the wire."* `FUN_141c7c900` writes its answer through out-parameters
   (`r8`/`r9` = `&action`, `&..`) rather than setting the mob's action itself, so a
   different caller asking it a question does not make a mob swing. `FUN_141ca7240` — the
   skill side — has 3 callers and the same shape.

   That distinction is why §1 is the load-bearing part of this file and §3.2 is only the
   mechanism. A static dominance argument over one function cannot rule out nine; 131 003
   reports with an empty attack range can.

### 3.3 The four client-side writers of state 4, and why none of them is us

The client can reach state 4 without the server. All four are in `CMob::Update`:

| at | gate | applies to our mobs? |
|---|---|---|
| `141c63494` | `mob+0xcd0 != 0` (`141c63481`) | **No.** `mob_change_controller` sends offset 74 as `-1`, and `141c541ab CMP EBP,-1 / JLE` is what skips `MOV [RDI+0xcd0],1`. `crates/net/src/mobmove.rs` has a test pinning that byte, for a different reason. **[D]** |
| `141c634ad` | `[r15 vtable+0x28]() == 0xa`, `r15 = mob+0x2c0 - 0x20` | **Unknown.** `[vtbl+0x28]` on the mob's second interface object is not decoded here. **[I]** that it does not fire — the three measurements in §1 say the block never ran, which is the evidence, not this row. |
| `141c634d8` | template id (`template+0x60`) ∈ `{0x83e803, 0x83e804, 0x83e82a}` = `8 644 611`, `8 644 612`, `8 644 650` | **No.** Every template ever spawned here is `1..1003` plus `800010`/`800011`. **[L]** |
| `141c637e6` | an RTTI check on `[rsi+0x20]` against the vtable at `0x1433A7B60`, **and** `mob+0x8a8 != 0`; and `141c63e74` clears `mob+0x8a8` at the end of the block | **Unknown**, same standing as row 2. **[I]** |

The honest form: **two of the four are not decoded.** What rules them out is not this
table, it is §1 — 131 003 reports, 257 hits and 78 opcodes all saying the block has never
run. The table's value is that it names what would have to be true for the one-line fix
in §5 to *not* be the whole story.

---

## 4. It is not the mob data, and it is not the geometry

* **The templates have the data.** Of the observed spawns, template `1003` is on the
  brief's list of the 65 (of 193) images with an `attack` node, and it has been fought.
  **[L]** via the brief's WZ enumeration; the spawn counts are mine.
* **The client checks the data itself.** `FUN_141c7c900` bails at `141c7c9a2
  cmp dword [rax+0x344], 0 / jle` — and `template+0x344` is written by the parser
  `FUN_14047d990` at `140486961`, in the window after the `lea` of the WZ name
  **`attack`** at `140486894`. So `+0x344` is the attack **count**, and "this mob has no
  attacks" is a gate the client owns. **[D]**
  *Control for that naming pass:* it reproduces `bodyAttack -> +0x7c` at store
  `14047f01d` from `lea 14047efea`, which `research/touch-damage.md` §5.3 read by hand.
  A second control (`targetFromSvr -> +0x1a0`) did **not** reproduce, so one of two
  controls passed; treat any *other* offset from that pass as a candidate.
* **`notAttack` is not parsed at all.** `tools/xref.py --string notAttack` finds one copy
  of the string and **0 code references**, while `bodyAttack` and `firstAttack` both
  resolve to `lea`s inside the same parser function. The snail's `notAttack 1` is
  therefore not what is stopping anything. **[D]** — blind spot: `xref.py` cannot see a
  string reached by an inline literal copy, which is a documented failure mode of that
  tool.
* **`research/touch-damage.md` §5.2 is now answered.** It said *"something has to register
  a body attack into `mob+0x9d0`, and I did not find that something"*. The registrar is
  `FUN_141cc8a20` (`template->GetAttack(idx)` at `141cc8a56`), and its three callers are
  `FUN_141c72bb0`, `FUN_141c912b0` and `FUN_141c937d0`. `FUN_141c72bb0` is called
  **twice from `FUN_141c69f40` itself** (`141c6b7fc`, `141c6b850`). The list is filled
  by the resolver as an attack animates — so an empty `mob+0x9d0` is a *consequence* of
  no attack ever being selected, not an independent cause. **[L]**

---

## 5. The change, and what to watch

`crates/world/src/session/combat.rs:59`

```rust
body: net::mobmove::mob_ctrl_ack(req.object_id, req.move_id, false),
//                                                           ^^^^^
```

is the only `mob_ctrl_ack` call in the workspace, and `git log -S` shows the argument has
been `false` since the packet was introduced in `17a19e4`. Whose message reads: *"Mobs
freeze because `0x02FF` needs an answer, **and the attack 'wall' was never measured**."*
It has now been measured, and it is that argument.

`crates/world/src/mobattack.rs` carries the decision and the decoder. **It is NOT wired** —
`crates/world/src/session/` belongs to the coordinator. One line:

```rust
body: net::mobmove::mob_ctrl_ack(req.object_id, req.move_id, world::mobattack::grant_attack()),
```

### Why unconditional is the right first variant

The client owns every other precondition and checks all of them itself: the attack count
(`template+0x344`), the per-attack cooldown (`[attack+0x9c]` added to `now` at
`141c63d09`, tested at `141c63923`), range and target (`FUN_141cdf5b0`), and the whole of
`FUN_141c7c900`. State 4 also survives only until the mob's next move report, so a grant
buys **one** action, not a mode. Adding a server-side rate limiter on top would change two
things at once, which `CLAUDE.md` forbids for exactly this reason.

### What to watch on the run, and what each outcome means

Nothing needs a probe — `world.log` alone answers it, because the client tells us:

| observation | meaning |
|---|---|
| a `0x02FF` whose body **offset 7** is in `0x1a..0x3b` | a mob chose `attack1..attackF`. **The gate was the flag.** Decode the index with `mobattack::decode_move_action` |
| a `0x02FF` with offset 7 in `0x3c..0x5d` | a mob chose `skill1..skillF` — but the ack sends skill id `0`, so `141c8214d TEST ESI,ESI / JE` short-circuits the skill block. Expect **none** until offsets 11/15 carry a real skill |
| a `0x00E5` with **attack index >= 0** | the numbered-attack damage path fired. Eight of the nine handlers in `touch-damage.md` §2.3 become live for the first time |
| **`0x0313` arrives** | independent confirmation the selection block ran, from the third layer |
| still nothing but `0xFF` and `0x0e`/`0x0f` | the flag is not sufficient. Then row 2 or row 4 of §3.3 is load-bearing, or something inside `FUN_141c7c900`'s 17 702 bytes refuses — and the next step is a probe on `141c639b7`'s return, not more static work |

**A warning about the second row.** Granting the flag also unlocks the *skill* arm, and
`0x03E4` offsets 11/15 are the skill id and level. They are `0` today, which is the safe
value: `141c8214b TEST ESI,ESI / JE 141c821c3` skips the whole skill block, and
`141c82151` even zeroes `mob+0x2f8` when the four `mob+0x9ac..0x9b8` flags say a skill is
already in flight. **Do not put a skill id in that packet in the same run.**

---

## 6. So: is 198 samples too few?

No — and the question does not arise. The failure is not a rare event that 131 003
samples happened to miss; it is a branch that has never been taken, for a reason that is
one `jne` wide and one `bool` deep. **A single client run with the flag flipped settles
it**, because a mob that attacks reports it in `0x02FF` offset 7 within seconds, and a
mob that does not is measurable in the same file by the absence of any byte outside
`{0x0e, 0x0f, 0xff}`.

If a number is wanted anyway: a mob picks an action roughly every 400 ms (131 003 reports
across the archive), so **one map with 30 mobs for 60 seconds is ~4 500 reports** — more
than an order of magnitude past what is needed to see a range that currently has zero
members.

---

## 7. Corrections to other files (not edited — recorded here as instructed)

1. **`research/touch-damage.md` §5.2** — "something has to register a body attack into
   `mob+0x9d0`, and I did not find that something". Found: §4 above. The direction is the
   other way round from what that sentence assumes.
2. **`research/mob-behaviour.md` §10.3** lists the `mob+0x2e4` writers and the 2/3/4
   values correctly and stops there. State `4` is the mob attack permission; §3 above.
   Nothing in that file is wrong, it just never asked.
3. **`CLAUDE.md`'s de-duplication rule needs a footnote.** "Deduplicate by content hash
   before counting anything across those two directories" is not sufficient — §1.1. The
   fixture copies of a run are frequently *longer* than the `previous-runs/` archive of
   the same run, and hash differently. Use `(timestamp, opcode, body)`.
