# Can the server draw a damage number over the player's head?

The owner, 2026-08-27: *"When I died to the Drakes, I still visually took 1 damage, but it wiped
out my whole HP bar. It should not show me 1 damage if what I received is not 1 damage."*

**Yes.** `0x02D1` effect `0x41` with a **negative** amount draws a violet number of the
server's choosing over the local player, through the identical renderer, digit set and code
path the client already uses for its own damage number. `crates/net/src/revive.rs`'s
`recovery_number(amount, delay_ms)` already builds it and already preserves the sign; its own
test asserts that. **No new packet builder is needed** — the wiring is one push in
`Session::on_user_hit`.

The one thing it does not do is *remove* the client's own `1`. §5.

Tags: **[L]** read off a listing, a table dump or a capture; **[D]** derived from two or more
[L]; **[I]** inferred.

---

## 0. The answers, up front

| question | answer |
|---|---|
| **Can the renderer be driven from the server?** | Yes, and it already is — the blue `+10` the owner confirmed is this exact function. **[L]** |
| **Can it be driven in a damage colour with a server value?** | Yes. The colour is chosen by the **sign** of one `i32` in the body. Negative → `NoViolet`, which is what the client's own damage number uses. **[L]** |
| **The packet** | `D1 02` `41` `<i32 -damage>` `<i32 delayMs>` `<i32 0>` — 15 bytes on the wire. §4 |
| **Does that fix the owner's sentence?** | It puts a *correct* number on screen. It does **not** delete the wrong one; both would draw. §5 |
| **Can a different spawn body make the client compute the real number instead?** | Almost certainly not, and this is now measured rather than argued: **224 captured `0x00E5` bodies across 22 runs and 8 mob templates whose WZ `PADamage` spans 3 to 287 all carry damage = 1.** §6 |

---

## 1. Instruments, and the control each one passed

Every number below came from a tool that was made to speak first, run with the repo as the
working directory (`CLAUDE.md` § *The scratchpad shadows the real tools*).

| instrument | positive control | result |
|---|---|---|
| `tools/listing.py` | `0x140304100` must show `raw@140304138`, `u8@140304144`, `u8@140304183`, then a `u16` run | reproduced exactly |
| `tools/callers.py` | `0x1402fa9a0` must give **96 sites in 15 functions, 43 of them in `0x140304b20`** | reproduced exactly |
| `tools/dump_va.py` | `0x140ee97cc 24` must give the six digit-set blocks `recovery-number.md` §4 lists | reproduced, entry for entry |
| `tools/rangescan.py 0x34f8` | must find `14277139c`, the stamp read by hand first | found (34 sites) |
| `tools/rangescan.py 0x544a` | must find `1428ac8b8`, read by hand first | found (11 sites) |
| the log decoder in §6 | must return **attack index `-1`** at body offset 4 — the value `research/user-hit.md` §3.2 derived from the listing, independently of any log | 224 of 224 |

That last one is the control that matters most for §6: it proves the body offsets are aligned
before any conclusion is drawn from the field beside them. A decoder reading 147 bytes at the
wrong offset would still print numbers.

**Ghidra was not used** (another agent holds the project lock). Everything here is capstone
over `client-patched\MapleStory.exe` through the repo's own tools, plus `previous-runs/`,
`research/fixtures/` and the live `world.log`.

---

## 2. `FUN_142771360` — what it takes, and every way in

`0x142771360 .. 0x142771939`, 1497 bytes, `research/msexe-shownumber.txt`.

```
FUN_142771360(CUser *pUser, i32 nAmount, i32 nOffsetIndex, i32 arg4, i32 arg5, i32 arg6)
```

`rbp = entryRsp - 0x4F`, so `[rbp+0x5f]` is the home slot of **arg2**, `[rbp+0x77]` is arg5
and `[rbp+0x7f]` is arg6. `nOffsetIndex` is multiplied by 20 (`lea edx,[r13*4]; add edx,r13d;
shl edx,2`) — it is vertical stacking, so two numbers at the same index land on the same
pixel. **[L]**

### 2.1 The sign is the whole mechanism

```
142771393  test edx, edx
142771395  jns  1427713ad
142771397  call 0x1429e3ef0            ; NEGATIVE ONLY: tick
14277139c  mov  [r12 + 0x34f8], eax    ;   lastDamagedTick
1427713a4  mov  byte [r12 + 0x34fc], 1 ;   "was damaged"
>1427713ad ... four shared early-outs at 1427713b4, 1427713c8, 1427713ec, 142771431 ...
142771489  test edi, edi
14277148b  jle  142771568
   POSITIVE (> 0):
14277154e  mov  dword [rsp + 0x20], 2  ; digit set 2
14277155e  call 0x140e0ccb0
>142771568  jns  14277166a             ; exactly 0 goes elsewhere - the MISS path
   NEGATIVE (< 0):
1427715ec  mov  edi, [rbp + 0x5f]      ; arg2, re-read from its home slot
1427715ef  neg  edi                    ; the magnitude is what is drawn
142771629  mov  dword [rsp + 0x20], 3  ; digit set 3
142771637  call 0x140e0ccb0
14277165e  mov  edx, edi               ; and, if the GM log is on:
142771660  call 0x142918b40            ;   "[Client] User Hit Damage: %d"
```

**[L]** The digit-set selector reaches a jump table at `0x140ee97cc`
(`tools/dump_va.py 0x140ee97cc 24`, reproduced today):

| index | block | canvases |
|---:|---|---|
| 0 | `140ee8f9d` | `NoRed0`, `NoRed1`, `NoCri0`, `NoCri1` |
| 1 | `140ee8fc2` | `NoRed2`, `NoRed3`, `NoCri2`, `NoCri3` |
| **2** | `140ee8fe7` | **`NoBlue0`, `NoBlue1`** ← positive |
| **3** | `140ee8ff7` | **`NoViolet0`, `NoViolet1`** ← negative |
| 4, 5 | `140ee9007` | `NoProduction0`, `NoProduction1` |

`recovery-number.md` §4 established that `FUN_142771360`'s positive branch is the **only** site
in the image that ever asks for set 2, out of eleven call sites of `FUN_140e0ccb0`. The mirror
matters here: **set 3 is the damage colour, and `FUN_142771360`'s negative branch asks for it**
at `142771629`. So "the blue number is drawn by the same renderer as the damage number" is not
an analogy — it is literally the same call with the sign flipped. **[L]** (That set 3 has three
other askers is `recovery-number.md` §4's classification of all eleven `FUN_140e0ccb0` sites;
I re-verified the jump table itself, not that classification.)

### 2.2 Every caller, all three kinds

`python tools/callers.py 0x142771360`, run today:

```
30 call sites in 23 functions
 0 tail jmp sites
 0 qword pointers in the image
```

That reproduces `recovery-number.md` §5 exactly. The `call`-only blind spot `CLAUDE.md`
records for `tools/callers.py` is closed here by the tool itself, which now reports all three
kinds separately — and both of the other two are zero, so the 30 `call` sites **are** the
complete reach set. **[L]**

Twenty-three of the thirty `neg` on the taken path — plain damage. Inside the `0x02D1` handler
`FUN_1427863f0` there are exactly three, which is what `callers.py` reports for that function
(`first 0x14278d89e`, `last 0x142790d67`), and §3 identifies which effect byte reaches each.

### 2.3 The negative-only side effect is safe, and that is measured, not argued

`user+0x34f8` / `user+0x34fc` are written **only** when the amount is negative. Enumerating
every use (`tools/rangescan.py 0x34f8`, control `14277139c` found; then `0x34fc`), the object-
pointer consumer that matters is `CUser::Update`:

```
14279fb60  mov eax, r12d              ; now
14279fb63  sub eax, [rsi + 0x34f8]    ; - lastDamagedTick
14279fb69  cmp eax, 0x1770            ; 6000 ms
14279fb6e  jl  → leave it
14279fb75  mov [rsi + 0x34f8], eax    ; re-stamp
14279fb7b  mov byte [rsi + 0x34fc], 0 ; and clear the flag
```

**[L]** It is a self-clearing six-second "recently damaged" timer.

And the reason this needs no further work: **the client already takes this branch on every
mob hit.** `FUN_1428aa0a0` reaches `FUN_142771360` at `1428aca14` with
`edx = ebx - r12d = -max(eax, 0)`, which is `≤ 0` for every input, and §6 counts **224
captured hits** where it drew a number. The stamp has therefore already run hundreds of times
in normal play with nothing observed. Sending a negative `0x02D1` does to the user object
exactly what the client does to itself. **[L]/[D]**

> **One use I did not resolve.** `FUN_142886870` reads `[r13+0x34f0]`, `[r13+0x34f4]`,
> `[r13+0x34f8]` and checks `ror(a ^ 0xBAADF00D, 5) + b == c` — an obfuscated-triple integrity
> check, which is *incompatible* with `+0x34f8` holding a raw tick. Either `r13` there is not
> a `CUser` (most likely — the same offset in a different object), or something is being
> checked that the client itself violates 224 times a session. I did not establish which.
> It does not gate anything on our side, because the write is the client's own, not ours.

---

## 3. The full `0x02D1` effect table, enumerated

`FUN_1427863f0` reads the effect byte first (`14278644e READ u8 → ebx`) and dispatches
**twice**.

**First switch**, `1427864a8 cmp ecx, 0x45` where `ecx = type - 8`, byte-index table
`0x142791300` (0x46 entries), target table `0x14279129C` (25 entries), both relative to image
base `0x140000000` (`lea rdx,[rip-0x27864a8]`). Dumped in full: of the **70** in-range types
(8..0x4D), **24 have their own block and 46 fall to the default** — and **`0x41`, `0x10` and
`0x24` are all in that 46**, index 24, the `default` at `0x14278bd20`, as are types 0..7 and
0x4E..0x54 via the `ja`. **[L]**

**Second switch**, in that default block: `14278bd7d cmp ebx, 0x54` (bytes `83 fb 54`,
verified against `tools/dump_va.py 0x14278bd75 20`), direct dword table at `0x142791348`,
**85 entries indexed by the raw effect byte**. **[L]**

### 3.1 The suppression gate, and it is now measured

Before the jump, the default block runs `recovery-number.md` §7's gate:

```
14278bd66  sub ecx, 0x4f  / je 14278bd7d    ; types 0x4F, 0x50, 0x51 skip the gate
14278bd6b  sub ecx, 1     / je 14278bd7d
14278bd70  cmp ecx, 1     / je 14278bd7d
14278bd75  test dl, dl
14278bd77  je   14279102e                   ; otherwise: discard the packet
```

`recovery-number.md` §9 listed "whether the §7 gate passes in normal play" as **unmeasured**.
**It has since been measured and it passes.** `previous-runs/` + `world.log` carry **41**
outbound `0x02D1 … effect 0x41` lines across 8 runs, and the owner reported *"I do see 10 in blue
above the character."*
That observation is a positive control for the entire chain — dispatch, both switches, this
gate, the queue, `CUser::Update`, and `FUN_142771360`'s four shared early-outs. **[L]**

### 3.2 Which effects reach the renderer

Enumerating all 85 second-switch entries and walking each block's instructions:

| effect | block | body | reaches the renderer how |
|---:|---|---|---|
| `0x10` (16) | `14278d881` | **one `u32`** | `mov edx, eax` — the value **verbatim**, immediately, `call 0x142771360` |
| `0x24` (36) | `14278d881` | **one `u32`** | the same block, same table, second key |
| `0x06`, `0x0a` | `14278d813` → falls through | `u32 id, u32 amount` | gated on `14278d875 cmp ebx, 0x3eba98` (4 111 000) — a hardcoded id |
| `0x23` (35) | `14278e204` | `u32 A, u8 flag, u32 B` | `A` verbatim, but `flag != 0` routes to a **different** renderer (`FUN_14291e000`, digit set 0) and `B` has a sign precondition |
| **`0x41` (65)** | `142790821` | **`i32 amount, i32 delayMs, i32 id`** | `FUN_14281e2d0` → queue on `pUser+0x39b8` → `FUN_14281e390` → `FUN_142771360` |
| `0x4b` (75) | `142790d30` | one `u32` (a char id) | `xor edx, edx` — always **0**, the MISS/guard display |

Every other entry either exits at `0x14279102e` or does something with no path to
`FUN_142771360`. The three direct sites found here are exactly the three `callers.py` reports
for this function, so the enumeration closes. **[L]**

> **The blind spot in that walk, named.** I bounded each block at the next block start in the
> table. That under-reports a block that *falls through* into another, and it demonstrably did
> so once: types `0x06`/`0x0a` enter at `14278d813` and fall through into `14278d881`, so their
> row above was recovered by reading, not by the walk. Any effect whose block falls through
> into a renderer block **from below the next table target** would be missed. The reason this
> does not weaken the conclusion: the *sites* are enumerated independently by `callers.py`
> (3 direct + 1 queue push, all four accounted for), so a missed row could only add another
> effect byte reaching a site already listed — it cannot add a route.

### 3.3 The queue is sign-blind, verified end to end

Push, `FUN_14281e2d0` (`research/msexe-numberqueue-push.txt`, 173 bytes, **one** call site in
the whole image, 0 tail jmps, 0 data pointers):

```
14281e2ea  mov  ebx, edx              ; nAmount - no test, no clamp, no abs
14281e2ef  call 0x1429e3ef0           ; tick
14281e2f4  mov  [rsp + 0x20], ebx
14281e2f8  add  eax, edi              ; tick + nDelayMs
14281e2fa  mov  [rsp + 0x24], eax
14281e33d  movsd [rax + 0x10], xmm0   ; node+0x10 = amount, node+0x14 = due  (bit-exact copy)
14281e342  mov  [rax + 0x18], ebp     ; node+0x18 = id
```

Drain, `FUN_14281e390` (**one** call site: `CUser::Update` at `0x1427a0139`):

```
14281e418  mov  ecx, [rdi + 4]        ; due
14281e41b  call 0x1408fc980           ; (tick - due) > 0
14281e424  mov  [rsp + 0x28], r12d    ; arg6 = 0
14281e429  mov  [rsp + 0x20], r12d    ; arg5 = 0
14281e42e  xor  r9d, r9d              ; arg4 = 0
14281e431  xor  r8d, r8d              ; arg3 = 0  (offset index)
14281e434  mov  edx, [rdi]            ; the amount, VERBATIM
14281e439  call 0x142771360
```

**[L] There is no sign test anywhere between the packet and the renderer.** A negative `i32`
in the body arrives at `142771393` unchanged and takes the `NoViolet` branch.

The three links each have a measurement behind them, which is unusual for this project and
worth stating plainly:

* the **shared prologue and the §7 gate** — proven by the blue `+10` drawing;
* the **negative branch of `FUN_142771360`**, including its two branch-local globals at
  `[rip+0x1336f16]` and `[rip+0x134e87b]` which the positive test does *not* cover — proven by
  the client drawing its own number 224 times through `1428aca14`;
* the **join** — read off the listing above.

What is **[D]** rather than [L] is the *composition*: no capture exists of a negative amount
arriving through the queue specifically. Every link is measured; the whole is not.

---

## 4. The packet

### 4.1 What to send

```
u16  opcode    0x02D1        // net::stats::USER_EFFECT_LOCAL
u8   effect    0x41          // net::revive::EFFECT_RECOVERY_NUMBER
i32  amount    -applied      // NEGATIVE -> NoViolet.  Positive would draw blue.
i32  delayMs   0             // fires on the first CUser::Update after now
i32  id        0             // only 0x13DA0E / 0x13DA0F add an extra animation
```

13 body bytes, 15 on the wire. For today's Drake hit that took 362:

```
D1 02  41  96 FE FF FF  00 00 00 00  00 00 00 00
```

`0xFFFFFE96 = -362`.

### 4.2 The builder already exists

`crates/net/src/revive.rs`:

```rust
pub fn recovery_number(amount: i32, delay_ms: i32) -> Vec<u8>
```

It writes `amount as u32` — sign-preserving — and its own test
`the_recovery_number_carries_a_signed_amount` already asserts `recovery_number(-7, 250)` keeps
`-7`, with the comment *"the sign must survive - it is what selects the colour"*. **Nothing in
`crates/net` needs to change.** The name is now half wrong; that is a rename, not a fix, and
`net` is not mine to edit.

### 4.3 Where it goes

`Session::on_user_hit`, `crates/world/src/session/combat.rs`. The handler already computes the
number this packet needs — the local `applied`, which is `incoming_damage_for(...)` falling
back to the client's claim. Push it **after** the `0x007C`:

1. `0x007C` `hp_only(chr.hp)` — unchanged, this is what moves the bar
2. `0x02D1` `recovery_number(-(applied as i32), 0)` — **unconditional**, not gated on the death
   transition
3. `0x0315` `show_revive_dialog()` — unchanged, still on `before > 0 && chr.hp == 0`

The `0x0315` ordering constraint is untouched: it gates on the client's copy of the HP that
the `0x007C` wrote, and `0x02D1` does not touch HP, so an intervening effect packet cannot
break it. **[D]**

Two things a first attempt gets wrong:

* **`applied` is a `u64` and the damage can exceed `i32::MAX` in principle.** Saturate rather
  than cast — a wrapped cast turns a huge damage into a *positive* `i32` and draws it blue.
* **Do not gate it on the transition.** Every hit gets a number. This is the effect-hangs-off-
  the-transition rule from `CLAUDE.md` read the other way: the *dialog* hangs off the
  transition, the *number* hangs off the hit.

### 4.4 Other players

The identical effect over a remote character is `0x02AF` with a `u32 charId` in front of the
effect byte — `FUN_1429bb720` reads that id at `0x1429bb745` before dispatching into the same
`FUN_1427863f0`. Not needed for the owner's sentence. **[L]**

### 4.5 The shorter alternative, and why not to use it first

Effect `0x10` is a **four-byte** body — one `i32`, drawn immediately with no queue and no delay
(§3.2). It is structurally simpler than `0x41` and reaches the same call. Verified against the
raw table (`tools/dump_va.py 0x142791348 340`): entry `[16]` and entry `[36]` are both
`0x0278d881`, entry `[65]` is `0x02790821`.

**Send `0x41` anyway.** `0x41` has been on the wire 38 times and drawn on the owner's screen; `0x10`
has never been sent and its `r15 = arg1` is [D] rather than [L] — argued from `r15` being
non-volatile across the default block's four calls, not from a run. One variant per launch, and
the one with a confirmed positive control is the one to spend it on.

---

## 5. What this does **not** fix, and it is the part worth saying out loud

**The client's own `1` is still drawn.** It is produced at *send* time inside `FUN_1428aa0a0`
(`user-hit.md` §4.4), before our packet exists, and nothing in a server reply un-draws it. The
visible result of §4 is therefore **two numbers over the head** — the client's `1` and our
`362` — not one corrected number.

They will not overlap exactly: the client's call passes `arg3 = [rbp+0x9e0]` as the offset
index, ours passes `0`, and the index is multiplied by 20 pixels. What that looks like on
screen is not predictable from here.

### The one gate that would suppress the client's number, and why I could not use it

`FUN_1428aa0a0` skips its whole draw block — including `1428aca14` — on either of two tests:

```
1428ac8a6  mov  ecx, 0xae
1428ac8ab  call 0x14090d160          ; an option lookup; non-zero -> skip
1428ac8b2  jne  1428acc0e
1428ac8b8  cmp  byte [r15 + 0x544a], r14b   ; r14b = 0
1428ac8bf  je   1428acc0e            ; the byte must be NON-ZERO to draw
```

**[L]** `tools/rangescan.py 0x544a` (control `1428ac8b8` found) returns **11 sites**: ten are
`cmp` inside the `0x00E5` builder family, and there is **exactly one write** —
`14205729b mov byte [rax+0x544a], bl` in `FUN_1420571b0`. That function has one caller
(`FUN_14202e460` at `0x14203834d`), no tail jumps, no data pointers, and
`tools/reads.py 0x1420571b0 2` finds **no packet reads in its subtree**.

**Hedged negative, with its blind spot named:** I found no route from a server packet to
`user+0x544a`. What that search could see: `[reg + 0x544a]` memory operands anywhere in
`0x142000000..0x143000000`, and packet-read primitives within two call levels of the single
writer. What it could **not** see: (a) a write through a pointer that was `lea`'d and handed to
a helper — the exact `mob+0x42c` failure `CLAUDE.md` records, and `rangescan` is structurally
blind to it; (b) the flag being *copied* from some other field that a packet does populate,
which a depth-2 read walk from the writer would not reach; (c) anything outside
`0x142000000..0x143000000`. So: **not found is not not-there.** If suppressing the client's own
number becomes the goal, that is where to start, and it needs a different instrument than the
one that came back empty here.

---

## 6. The cause: can a different spawn body make the client compute the real number?

This is the route that would fix the disease rather than the symptom. **The measurement says
no**, and it is worth more than the static trace beside it.

### 6.1 The enumeration

Every `0x00E5` body in `previous-runs/world*.log` and the live `world.log`, decoded with the
offsets `user-hit.md` §3 derived from the listing. The control: **attack index at offset 4 must
be `-1`** — the value that document derived from the builder's mob arm, with no log involved.
It is `-1` in 224 of 224, so the alignment is right before anything else is read.

```
22 capture files, 224 bodies, every one exactly 147 bytes
attack index @4 : -1        x224
damage       @8 :  1        x224
```

Split by the mob that did the hitting, against that mob's `PADamage` in the client's **own**
`Mob.wz` (`gm-handbook/mobtemplates.txt`, generated by `tools/dump_mobs.py`):

| template | hits | max HP (WZ) | **PADamage (WZ)** | **damage the client reported** |
|---:|---:|---:|---:|---:|
| 2 | 172 | 45 | 3 | **1** |
| 3 | 14 | 51 | 5 | **1** |
| 4 | 3 | 62 | 7 | **1** |
| 5 | 10 | 68 | 9 | **1** |
| 6 | 5 | 133 | 11 | **1** |
| 7 | 11 | 115 | 13 | **1** |
| 10 | 6 | 172 | 27 | **1** |
| **45** | 3 | 3218 | **287** | **1** |

**[L] A 96-fold range in the input and a constant output.** `STATUS.md`'s discriminator was
*"one hit from a much stronger mob"*; this is eight mobs, and the answer does not move.

And the owner's exact sentence is in today's `world.log`, one line, both halves of it:

```
hit by mob 2004 (template 45, attack index -1) for 362 - hp 238 -> 0. The CLIENT claimed 1
```

One hit, 238 HP to zero, the client's own number `1`. That is not a recollection reconstructed
from two sessions — it is a single log line. **[L]**

### 6.2 Why, structurally

The damage that reaches the wire is the local `[rbp-0x60]` in `FUN_1428aa0a0`, stored into the
HITINFO at `1428ad4ca` from `r12d`, which is **reloaded** from that local 0x39 bytes earlier at
`1428ad491`.

> This corrects `research/user-hit.md` §3.3, which reads *"`r12d` is argument 2… It goes
> straight into the HITINFO at `1428ad4ca`."* `r12`/`r12d` is written **26 times** in this
> 14 497-byte function; the value stored is the running local, not the argument. §3.3's
> *conclusion* —
> that struct `+0x08` is the damage — is unaffected and is confirmed by all 224 captures.

`[rbp-0x60]` is written nine times. Two are floors:

```
1428aaa28  test r12d, r12d
1428aaa2b  sete bl
1428aaa2e  mov  [rbp], ebx        ; [rbp] = "this was a miss"
...
1428ab94e  cmp  dword [rbp], 0
1428ab952  jne  1428ab966         ; a miss stays 0 and draws the MISS display
1428ab956  cmp  r12d, ecx         ; ecx = 1
1428ab959  cmovg eax, r12d
1428ab960  mov  [rbp - 0x60], eax ; damage = max(damage, 1)
```

**[L] There is an explicit `max(damage, 1)` floor**, skipped only for a miss. So `1` is what
this function emits whenever its computed damage is `≤ 1` and non-zero.

**[L]** Separately: **every** mob-family call site into `FUN_1428aa0a0` — all **16 sites in the
14 functions** `0x141c69f40`, `0x141c6cec0`, `0x141c6d1d0`, `0x141c6d690`, `0x141c6dccf`,
`0x141c6de40` (×3), `0x141c6f830`, `0x141c70a00`, `0x141c70e00`, `0x141c712c0`, `0x141c71f60`,
`0x141c724c0`, `0x141cc6000`, `0x141d060b0` — passes `xor edx, edx`, i.e. `nDamage = 0`. Read
by walking each caller's listing for every `call 0x1428aa0a0` and asserting the count came back
to `callers.py`'s 26 across all 23 callers, not from the `first`/`last` columns.

**[D]** The damage the builder works with therefore comes from inside, via the 64-bit pair
returned by `FUN_140265f00` / `FUN_1402661b0` into `[rbp+0x30]`/`[rbp+0x34]` at `1428aa84d`.
**I did not trace inside those two functions.** The 224-body table is the stronger instrument
and it says whatever they compute does not depend on which mob hit you.

### 6.3 The one server lever that has never been pulled

The spawn packet carries a **`forcedStatPresent`** byte at body offset 10
(`141d33734`), and `crates/net/src/mob.rs` sends it as **`0`** for every mob. Its parser
`FUN_14085acd0` (`research/msexe-mobforcedstat.txt`) reads a `str` then **twelve `u32`s** into
`+0x10 … +0x48` and a trailing `u8` into `+0x04` — a server-side stat *override* block.

**[I]** In this game family that block is where a server overrides a mob's WZ stats, attack
power included. **Which of the twelve `u32`s is attack power is not established here**, and
neither is whether the hit path reads the override at all.

**So the honest statement of the negative is:** the client's reported damage does not track the
mob's WZ `PADamage` — measured, 224 bodies, 8 templates, 96× spread. It does **not** follow that
no spawn body could move it, because the one field that could plausibly move it has never been
sent non-zero. Reading it as *"nothing the server sends can affect the number"* would be exactly
the over-claim `CLAUDE.md` § *"Not found is not not-there"* is about. What is established is
that the **route in §4 is available now and needs no such experiment.**

The counter-argument, for whoever picks that up: the client *can* already read this mob's WZ
stats — `mob+0xb60 = hp * 100 / <template max HP>` at `141c50502` divides by the template's own
`maxHP`, and the HP bar draws at the right percentage, so the template is loaded and its numeric
`info` fields are reachable. `PADamage` sits in the same node. **[D]** That makes "the client
cannot see the mob's attack power" false, and points at "the hit path does not use it" — which
would make the forced-stat block useless too. Not settled either way.

---

## 7. What I did **not** establish

* **That the queued negative draws.** Every link is measured (§3.3) but no capture exists of a
  negative amount arriving through `0x02D1`/`0x41` specifically. This is the one thing a launch
  would settle, and §8 says what each outcome would mean.
* **What the two numbers look like together** (§5). The client's `1` will still be drawn. I have
  no way to predict the on-screen result and the owner can see it in one glance.
* **Any server route to `user+0x544a`**, the flag that would suppress the client's own number.
  §5 names three things that search could not have seen.
* **What `FUN_140265f00` / `FUN_1402661b0` compute** (§6.2) — the client's own damage
  calculator. Not opened.
* **Which of the forced-stat block's twelve `u32`s is attack power**, or whether the hit path
  reads the override at all (§6.3).
* **Whether `FUN_142886870`'s `[r13+0x34f8]` is the same field** `FUN_142771360` stamps (§2.3).
  Two readings of the same offset that cannot both be right; I did not identify `r13`.
* **The colour of the client's current `1` on screen.** Static reasoning says `NoViolet`
  (`1428aca14` passes a value that is `≤ 0` for every input, and `< 0` selects set 3). The owner has
  never been asked what colour it is. **[D]**
* **What `nId` other than `0x13DA0E`/`0x13DA0F` does** — nothing on this path, per
  `recovery-number.md` §9, which I did not re-check.

---

## 8. If a launch is spent on it

One variant. `0x02D1` effect `0x41` with a negative amount, on top of the `0x007C` that already
works. Every outcome is a claim that can come back false:

| on screen | reading |
|---|---|
| a **violet** number equal to what the bar lost, beside or above the client's `1` | done. §4 confirmed end to end |
| a **blue** number | the amount went out positive — a cast or a `u32`/`i32` slip in the wiring, not the packet shape |
| **only the `1`**, and `client-patched\maplecw-hook.log` has **no** dispatch line for the new `0x02D1` | framing, not this. Count `0x02D1` in `world.log` and in the hook log and compare |
| **only the `1`**, and the hook log **has** a dispatch line that returned | not the §7 gate — the blue `+10` already proved that passes. Look at the queue: `delayMs`, or the drain not running |
| the client freezes | not this packet. It is server→client, needs no reply, and the handler's only allocation is the 0x20-byte node |

The dispatch line is written on **return**, so a *missing* line with the packet present in
`world.log` means the handler was entered and did not come back — count it in both files.

---

## 9. Reproducing every number here

Repo root as the working directory, always.

```
python tools/listing.py  0x140304100 | grep READ        # control
python tools/callers.py  0x1402fa9a0                    # control: 96 sites, 43 in 0x140304b20
python tools/dump_va.py  0x140ee97cc 24                 # control: the digit-set table

python tools/callers.py  0x142771360     # 30 sites / 23 fns / 0 tail jmp / 0 data ptr
python tools/callers.py  0x14281e2d0     # ONE site: 0x142790848
python tools/callers.py  0x14281e390     # ONE site: 0x1427a0139 (CUser::Update)
python tools/callers.py  0x1428aa0a0     # 26 sites / 23 fns
python tools/listing.py  0x1427863f0     # the 0x02D1 handler, 10 195 instructions
python tools/dump_va.py  0x142791300 70  # first switch, byte index table (idx = type - 8)
python tools/dump_va.py  0x14279129C 100 # first switch, 25 target blocks
python tools/dump_va.py  0x142791348 340 # second switch, 85 blocks keyed by the raw type
python tools/dump_va.py  0x14278bd75 20  # the bytes of `cmp ebx, 0x54`
python tools/rangescan.py 0x34f8 0x140001000 0x143000000   # control 14277139c; 34 sites
python tools/rangescan.py 0x544a 0x142000000 0x143000000   # control 1428ac8b8; 11 sites, 1 write
python tools/reads.py    0x1420571b0 2   # the 0x544a writer: no packet reads
```

The 224-body table in §6.1 is a one-off decoder over
`previous-runs/world*.log` + `world.log`: match `0x00E5 … <n> byte body <hex>`, then read
`i32 @4` (attack index — **the control, must be `-1`**), `i32 @8` (damage) and `u32 @113`
(mob template), joining the template against `gm-handbook/mobtemplates.txt`. Bodies in
`research/fixtures/*.log` are byte-identical copies of `previous-runs/` entries and were
deduplicated out, so 224 is a count of distinct captures, not of files.

**Do not read `callers.py`'s `first`/`last` columns as the site list** — four of
`FUN_142771360`'s 23 callers have more than one site. `recovery-number.md` §5's warning box
still applies and the block-level walk in §3.2 was written to obey it.
