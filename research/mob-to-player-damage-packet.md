# When a mob hits the player: what draws the number, and how the server sets it

**2026-08-28.** The owner: *"this is the official client and this behavior does not exist (showing 2
damage numbers by mobs) means that something is wrong with our implementation."*

They are right, and this file answers why in a way that makes the fix mechanical.

Markers: **[L]** read off this client's listing, its WZ or a capture; **[D]** derived from two or
more [L] facts; **[I]** inferred.

---

## 0. The answers, up front

| question | answer |
|---|---|
| **Is there an inbound packet that draws a mob's damage on the player?** | **No.** Five opcodes can reach the damage renderer; none of them is a mob hit. §2, §3 |
| **So how does the server set the number?** | It sends the mob's **attack power**, not the damage. The client multiplies it by its own random roll and the player's defence, draws the result, and *reports* it back in outbound `0x00E5`. §4 |
| **Where does the attack power come from?** | `PADamage` in the client's own `Mob.wz` -> `template+0x34` -> `*(mob+0x3c8)+0x58`. **Or** the server overrides it with `0x03C6`'s `forcedStatPresent` block, wire field 3. §4, §5 |
| **Why is ours always `1`?** | `1.0` is the **floor inside the client's own formula**, and `pad = 0` is the only input that produces it with zero variance. 198 of 198 captures show exactly that signature. Which link delivers the zero is **not** established. §6 |
| **Which renderer caller does our `0x02D1` use?** | `FUN_14281e390`, the queue drain - **not** one of `FUN_1427863f0`'s three direct sites. §3.3 |
| **What does the coordinator do next?** | One launch, one variant: send `forcedStatPresent = 1` with a large `pad`. Two opposite readings, §7 |

**This file corrects `damage-number-suppress.md` §5.3/§5.5**, which concluded the client has *no
live reader for a mob's attack power*. It has one, it is on the contact-damage path, and the
override route the same section declared dead is the live one. §5.4 says exactly how that
negative went wrong, because the shape of the error is one `CLAUDE.md` already names.

---

## 1. Instruments, and the control each one passed

| instrument | positive control | result |
|---|---|---|
| `tools/callers.py` | `0x1402fa9a0` must give **96 sites in 15 functions, 43 in `0x140304b20`**, 0 tail jmps, 0 data ptrs | reproduced exactly |
| `find_handler_table.build_callgraph` | must independently reproduce `callers.py`'s **23** renderer-caller functions | reproduced, function for function |
| the local-user case-block map (§2) | block for opcode `0x2d1` must resolve to `FUN_1427863f0`, the handler three prior files already name | reproduced |
| `tools/dataref.py` | the `maxHP` slot must have references (the HP bar works) | 2 reads, both in `FUN_14047d990` |
| `tools/listing.py` / `tools/dump_va.py` | as used throughout, cross-checked against each other at `0x141c4d331` | agreed |
| the string search (§4.1) | must find `fixedBodyAttackDamage` at `0x14328afb8`, a documented **[L]** | **failed, then fixed** - see below |
| `cargo test -p net` | 464 tests | pass, clippy clean |

**Ghidra was used** (I held the project lock). Decompilation is in
`research/msexe-damagecalc.txt`, `research/msexe-mobattackpower.txt` and
`research/msexe-mobstat-object.txt`.

### 1.1 Two instruments lied to me first, and both were caught by their control

Worth recording, because in both cases the wrong answer was clean and confident.

**A VA/RVA mix-up returned "0 of 179 game-stage opcode handlers reach the renderer".** The
handler table was keyed by full VA and tested against a reachability set keyed by RVA, so
nothing ever matched. The control that broke it was not about the renderer at all: *"how big is
a game-stage handler's forward subtree?"* came back **min 1, median 1, max 1**, which is
impossible for a 179-function set. Corrected, the answer is **32**, and §3.2 explains why even
that number is noise rather than a finding.

**A byte search for `PADamage` reported it absent from the image.** The control -
`fixedBodyAttackDamage`, whose address `damage-number-two-numbers.md` §4 records as **[L]** -
was absent too, which is what said the search was broken rather than the subject. These
property names are **UTF-16**; `PADamage` is at `0x1432b16b0`. Had the control been skipped, the
file would have concluded the client cannot see a mob's attack power - the same wrong answer
`damage-number-suppress.md` reached by a different route.

---

## 2. Every way into the renderer, enumerated

`FUN_142771360` is the damage-number renderer (`damage-number-draw.md` §2). `tools/callers.py`:

```text
30 call sites in 23 functions
 0 tail jmp sites
 0 qword pointers in the image
```

Both other kinds are zero, so the 30 `call` sites **are** the complete reach set. **[L]**
`damage-number-suppress.md` §3.2 splits those 23 functions into **8 that carry a `user+0x544a`
gate** and **15 that do not**. The brief asked for those fifteen; here they are, with the one
fact that matters about each.

| # | function | reachable from a packet handler? |
|---:|---|---|
| 1 | `0x14185ad30` | no dispatcher ancestor within 10 levels |
| 2 | **`0x1427863f0`** | **yes - the `0x02D1` handler**, called directly by the local-user dispatcher |
| 3 | `0x142798c70` | no |
| 4 | `0x142799290` | no |
| 5 | `0x1427995c0` | no |
| 6 | `0x1427a5a20` | no |
| 7 | `0x1427a9d50` | no |
| 8 | **`0x14281e390`** | indirectly - the `0x02D1` **queue drain**, run from `CUser::Update`. §3.3 |
| 9 | `0x14288d340` | no |
| 10 | `0x14288d4d0` | no (0 ancestors of any kind) |
| 11 | **`0x14289a3a0`** | **it IS the local-user dispatcher**, and it draws inline for one opcode. §3.1 |
| 12 | **`0x1428a3bf0`** | **yes - opcode `0x02F3`** |
| 13 | **`0x1428ee7d0`** | **yes - opcode `0x02F2`** |
| 14 | **`0x142903600`** | **yes - opcode `0x0313`** |
| 15 | `0x1429d48c0` | no |

### 2.1 The local-user dispatcher, and how the opcodes were resolved

`FUN_14289a3a0` - already on disk as `research/msexe-localuser-dispatch.txt`, and never before
read as a dispatcher - switches on opcodes **`0x2c5..0x39e`** plus `0x62b`:

```asm
14289a3e1  cmp  edx, 0x62b
14289a3f3  lea  eax, [rdx - 0x2c5]
14289a3f9  cmp  eax, 0xd9
14289a3fe  ja   0x14289d5f5            ; default
14289a406  lea  rcx, [rip - 0x289a40d] ; = 0x140000000
14289a40d  mov  edx, [rcx + rax*4 + 0x289d660]
14289a417  jmp  rdx
```

**[L]** 218 dword entries at `0x14289d660`, each an offset from image base; 199 opcodes reach
174 distinct blocks, the rest go to the default at `0x14289d5f5`. Mapping each renderer-bearing
call site to the greatest block start at or below it gives:

| call site | callee | block | **opcode** |
|---|---|---|---:|
| `0x14289a43f` | `FUN_1427863f0` | `0x14289a439` | **`0x02D1`** |
| `0x14289ad10` | `FUN_1428ee7d0` | `0x14289ad0a` | **`0x02F2`** |
| `0x14289ad20` | `FUN_1428a3bf0` | `0x14289ad1a` | **`0x02F3`** |
| `0x14289b549` | `FUN_142903600` | `0x14289b543` | **`0x0313`** |
| `0x14289b63f` | `FUN_142771360` **direct** | `0x14289b553` | **`0x0314`** |

The control for this table is its first row: `0x2d1 -> FUN_1427863f0` is the handler
`damage-number-draw.md` §3 identified independently, and the block map re-derives it. **[L]**

---

## 3. None of the five is a mob hit

### 3.1 `0x0314` and `0x0313` are self-damage, not reports of damage

`0x0314`'s block reads **no packet fields at all**. It computes a value from the local user,
builds an **outbound** packet, sends it, and only then draws:

```asm
14289b5dc  mov  dword [rbp-0x2c], 0xfffffff5   ; attack index -11
14289b5e3  mov  dword [rbp-0x28], edi          ; the damage IT computed
14289b5f7  mov  edx, 0xe5
14289b603  call 0x1406ed520                    ; COutPacket(0x00E5)
14289b620  call 0x1415d01c0                    ; SendPacket
14289b625  neg  edi                            ; negative -> NoViolet, the damage colour
14289b63f  call 0x142771360
```

**[L]** That is the *same shape* as the mob-contact path: the client decides the number, draws
it, and tells the server. `0xfffffff5` = **-11** sits in the attack-index field at body offset 4
- the field whose value is `-1` in all 198 archived contact hits. So `0x0313`/`0x0314` are
server-**triggered** damage of some other kind (the two differ only in which value they compute),
and they confirm the protocol's direction rather than offering an alternative to it.

`0x0313` (`FUN_142903600`) has the identical build-send-`neg`-draw sequence at
`0x142903c28`..`0x142903c64`. **[L]**

### 3.2 The descent from the game stage adds nothing

Descending from all **179** game-stage case handlers, **32** reach the renderer within 8 call
levels - but every one of those paths is 5 to 8 levels deep and passes through shared machinery
(`CUser::Update` reaches `FUN_14281e390` at `0x1427a0139`, and `CUser::Update` is reachable from
almost everything). The 23 direct callers are the authoritative set; these are call-graph noise.
I report the number rather than suppressing it so the next person does not re-derive it and
think it means something.

> **Blind spot, named.** `build_callgraph` scans for direct `E8`/`E9 rel32` only and keeps a
> target only when it lands on a `.pdata` entry. It cannot see a virtual call, and the two
> dispatchers are themselves reached virtually - `FUN_14289a3a0` has 36 ancestors and **no**
> dispatcher among them. So "no packet handler reaches function X" here means *no direct-call
> path*, and a vtable hop would be invisible. The five positives in §2.1 are unaffected: they
> were found by mapping call sites to switch blocks, which does not use the call graph at all.

### 3.3 Which renderer caller our `0x02D1` uses - the brief asked explicitly

**`FUN_14281e390`, at `0x14281e439`.** Not one of `FUN_1427863f0`'s own three direct sites.

Effect `0x41` does not draw from the handler. It pushes a node onto the queue at `pUser+0x39b8`
via `FUN_14281e2d0`, and `CUser::Update` drains it through `FUN_14281e390`
(`damage-number-draw.md` §3.2/§3.3, re-checked against `callers.py`: each of those two functions
has exactly **one** call site in the image). **[L]**

That matters for the replacement: `FUN_14281e390` is **ungated** - it carries no `user+0x544a`
test - so anything done to suppress the client's own number leaves ours drawing. The two are
independent, which is exactly why both appear on screen.

---

## 4. The real mechanism: the server sends attack power, the client makes the number

### 4.1 `PADamage` -> `template+0x34`

The mob template loader `FUN_14047d990` reads the interned name `PADamage` (UTF-16 at
`0x1432b16b0`, reached through the pointer slot `0x143a48078`) and stores:

```asm
140480ab4  call 0x14022ee40          ; property -> int, default 0
140480ab9  test eax, eax
140480abb  cmovs eax, r12d           ; negative -> 0
140480ac8  cmp  eax, 0x1869f
140480acb  cmovl ecx, eax            ; clamp to 99999
140480ace  mov  dword [r13 + 0x34], ecx   ; <<< template + 0x34
```

**[L]** The same function puts `acc` at `template+0x4c` (`0x140480d61`) and `eva` at
`template+0x50` (`0x140480dda`).

And the values are really there: `gm-handbook/mobtemplates.txt`, generated by `tools/dump_mobs.py`
from this client's own `Mob.wz`, gives template 2 (the snail on map 40) `PADamage 3, acc 33,
level 1` and template 45 `PADamage 287`. **[L]**

### 4.2 `template+0x34` -> the mob's stat object

`FUN_14046ad90(statObj, template, pct, forcedStat)` - 522 bytes, decompiled - is the copy:

```c
if (param_2 != 0) {                                   // the template
    param_1[0x16] = FUN_14047a190(param_2);           // +0x58 <- tpl+0x34  = PADamage
    param_1[0x20] = FUN_14047a1a0(param_2);           // +0x80 <- tpl+0x30  = MADamage
    param_1[0x2a] = FUN_14047a1d0(param_2);           // +0xa8 <- tpl+0x4c  = acc
    param_1[0x2e] = FUN_14047a1e0(param_2);           // +0xb8 <- tpl+0x50  = eva
    *param_1      = FUN_14047a1f0(param_2);           // +0x00 <- tpl+0x48  = level
    ...
    if (param_3 != 100) {                             // a percentage scaler
        param_1[0x16] = (param_3 * param_1[0x16]) / 100;
        param_1[0x20] = (param_3 * param_1[0x20]) / 100;
    }
    if (param_4 != 0) { /* the override - section 5 */ }
}
```

`statObj` is `*(mob+0x3c8)`; `template` is `mob+0x3a8`. Both are set in `FUN_141c4cee0` well
before the call (`param_1[0x75] = param_2` at its line 197, the call at line 906), and
`mob+0xA20` is explicitly zeroed at line 550. **[L]**

There are **four** call sites, and the three on the spawn path all pass `mob+0x3a8` and
`mob+0xA20` the same way (`0x141c4e75d`, `0x141c53418` in the 106-byte `encodeInit`
`FUN_141c4ff80`, `0x141c76256` in the 20-byte `FUN_141c76190`). **[L]**

> **`calcDamageIndex` is not the percentage.** I expected the spawn packet's byte 5 to be
> `param_3`, which would have meant our `1` was scaling every mob's attack to 1%. It is not:
> `FUN_141c76190` saves that byte in `r15d` and passes **`0x64` (100)** unless
> `FUN_141866780` overrides it (`0x141c76241 mov eax, 0x64`). Recording the dead end because
> it is an attractive wrong answer that fits the symptom.

### 4.3 The stat object -> the damage

`FUN_14025e540(statObj, attackRecord)` returns the physical attack power, and its main line is
`*(int *)(param_1 + 0x58)` - the field §4.2 just filled. `FUN_140265f00` then computes, with
every constant read out of `.rdata`:

```text
r      = rand * 2^-32 * 0.4 + 0.1        in [0.1, 0.5)
raw    = (r + 1.0) * pad                 in [1.1*pad, 1.5*pad)
raw   *= 1.0 - pdd / ((pdd + (playerLevel + 40) * 5) + raw * 1.2)
damage = clamp(raw, 1.0, 50000000.0)
```

**[L]** `2.3283064365386963e-10` @ `0x14327a9c8`, `0.4` @ `0x14327aa18`, `0.1` @ `0x14327a9f0`,
`1.0` @ `0x1434b95e0`, `1.2` @ `0x14327aa48`, `5.0e7` @ `0x14327ab18`, `100.0` @ `0x143277e78`.

`FUN_1402661b0` is the same function for **magic** (`statObj+0x80`, and the player's defence at
`+0x14` instead of `+0x0c`); `FUN_1428aa0a0` picks between them on the attack record's `+0x84`.
For contact damage the record is null, so the physical one always runs. **[L]**

`crates/net/src/mobdamage.rs`'s `predicted_client_damage` is that formula, with tests.

---

## 5. The server's lever: `forcedStatPresent`, and it was wrongly written off

### 5.1 What the block overrides

Same function, the tail:

```c
if (param_4 != 0) {                                  // mob + 0xA20
    param_1[0x16] = *(u32 *)(param_4 + 0x18);        // <<< PAD  - the damage input
    param_1[0x20] = *(u32 *)(param_4 + 0x1c);        //     MAD
    param_1[0x1a] = *(u32 *)(param_4 + 0x20);
    param_1[0x24] = *(u32 *)(param_4 + 0x24);
    param_1[0x2a] = *(u32 *)(param_4 + 0x28);        //     acc
    param_1[0x2e] = *(u32 *)(param_4 + 0x2c);        //     eva
    if (0 < *(int *)(param_4 + 0x38)) param_1[0x32] = *(int *)(param_4 + 0x38);
    *param_1      = *(u32 *)(param_4 + 0x30);        //     LEVEL
    *(u64 *)(param_1 + 0x128) = *(u64 *)(param_4 + 0x50);
}
```

**[L]** It is **not a merge**. Once the block exists, level, acc and eva come from it too - and
the client's hit-type function `FUN_140268140` divides by `(playerLevel - mobLevel + 51) * 5`
and tests `acc` against the player's evasion. A block that sets `pad` and leaves the rest zero
changes four inputs, not one. `MobForcedStat::from_template` exists for exactly that reason.

> The consumer reads `forcedStat + 0x50`; the parser never writes it. Whatever the allocation
> at `0x141cc9438` leaves is what gets copied. Not investigated. **[L]**

### 5.2 The wire format, from the parser's own listing

`FUN_14085acd0`, `0x14085acd0..0x14085ad8b`, 187 bytes - **57 body bytes**, and the wire order
is **not** the struct order (fields 9, 10, 11 land at `+0x34`, `+0x38`, `+0x30`):

| wire | type | struct | meaning |
|---:|---|---|---|
| 0 | `u64` | `+0x08` | max HP |
| 1 | `u32` | `+0x10` | no traced consumer |
| 2 | `u32` | `+0x14` | no traced consumer |
| **3** | `u32` | `+0x18` | **physical attack power** |
| 4 | `u32` | `+0x1c` | magic attack power |
| 5 | `u32` | `+0x20` | physical defence |
| 6 | `u32` | `+0x24` | magic defence |
| 7 | `u32` | `+0x28` | accuracy |
| 8 | `u32` | `+0x2c` | evasion |
| 9 | `u32` | `+0x34` | no traced consumer |
| 10 | `u32` | `+0x38` | copied only when `> 0` |
| 11 | `u32` | `+0x30` | **level** |
| 12 | `u32` | `+0x48` | no traced consumer |
| 13 | `u8` | `+0x04` | stored as `!= 0`; no consumer traced |

**[L]** `FUN_141cc9410` allocates, stores `alloc+0x28` at `mob+0xA20`, and tail-jmps into the
parser reading nothing else - so 57 bytes is exact.

### 5.3 It costs nothing today

```asm
141d33734  call 0x1406e8ae0      ; READ u8  forcedStatPresent
141d33739  test al, al
141d3373b  je   0x141d33748      ; ZERO -> skip FUN_141cc9410 entirely
141d33743  call 0x141cc9410
```

**[L]** Our `0` means the block is never parsed and `mob+0xA20` stays null, so the client uses
its own WZ template - which is the correct default. Sending `1` adds exactly 57 bytes.

### 5.4 Why `damage-number-suppress.md` §5 got the opposite answer

That section concluded: *"ten of the twelve `u32` fields are read by accessor functions that
nothing in the image calls"*, therefore *"the client contains no live reader for the mob's attack
power at all"*, therefore the route is dead. Two separate errors, and both are shapes
`CLAUDE.md` already warns about.

**The fields are not read through those accessors.** `FUN_14046ad90` reads the block by
**direct field offset** off a pointer passed in as an argument. Counting callers of the
accessors could never have found it. This is precisely the blind spot §5.4 of that file *named*
- *"the `mob+0xA20` pointer loaded once into a register or local far from its uses and
carried"* - and `FUN_141c4cee0` does exactly that: `uVar13 = param_1[0x144]` at its line 900,
handed to `FUN_14046ad90` at line 906. The blind spot was written down correctly and then the
conclusion was drawn as though it were closed.

**And the template accessors are not dead either.** That file reports `0x14047a190 (tpl+0x34) :
1 caller` and reads the one caller as "its own dead accessor". `tools/callers.py` today:

```text
0x14047a190: 1 call site  (0x14046ad90)  +  2 TAIL JMP sites:
                0x14025e540 at 0x14025e571   <- the live contact-damage calculator
                0x141c8a867                  <- the forced-stat shim
```

The single `call` is the live initialiser, and there are **two tail jumps on top of it** - the
`callers.py` blind spot `CLAUDE.md` records by name. Same for `0x14047a1a0`.

The generalisation is this file's own §1.1 repeated: **the wrong answer was clean and
confident.** Nothing about "0 call sites" announces that the question was wrong.

---

## 6. So why is it 1 - what is settled and what is not

**Settled [L]:** `1.0` is a floor *inside the client's own damage formula*
(`raw.clamp(1.0, 5e7)`, §4.3). It is not a stub, not a debug path, and not something a packet
turns off.

**Settled [D]:** `pad = 0` produces exactly `1` for every random roll and every player defence -
the only input that yields a constant with **zero variance**. `PADamage 3` cannot look like it
(minimum 2 after defence), and `PADamage 287` produces a number in the hundreds. That is
asserted as a test in `mobdamage.rs`. Against the measurement of **198 hits over 160 archived
world logs, every one damage 1, across 8 templates whose `PADamage` spans 3 to 287**
(`damage-number-two-numbers.md` §3), the attack power reaching the calculator **is zero, or the
damage is being zeroed after it is computed**.

**Not settled - and I want to be exact about this, because the static chain is long.** There are
two ways to get a hard zero and I did not discriminate them:

1. **The attack power is 0.** But §4.2 shows the template pointer and the forced-stat pointer are
   both correctly initialised before every one of the three spawn-path calls, and §4.1 shows the
   WZ really carries `PADamage 3`. I could not find where this would go to zero.
2. **The damage is zeroed after the fact by the hit-type gate.** `FUN_140265f00` ends with
   `param_2[1] = (*param_2 == 1) ? damage : 0` - the damage field is **forced to 0 unless
   `FUN_140268140` returned 1**. That function returns `2` or `3` on evasion/dodge outcomes
   computed from the mob's `acc` and level against the player's evasion at `playerStat+0x1c` and
   a field at `playerStat+0x10`. **[L]** `FUN_1428aa0a0` then applies `max(damage, 1)` at
   `0x1428ab956`, turning that 0 into the `1` on screen.

Reading (2) fits the evidence at least as well as (1) and would point at **a player stat we
send**, not at the mob. I did not trace `FUN_1428aa0a0`'s `[rbp-0x60]` back to either
`param_2[1]` or `param_2[2]`; that is a 14 497-byte function and the local is written nine
times. **This is the single biggest thing left open**, and §7 is designed to decide it.

---

## 7. How to spend one client run

**One variant.** Send `forcedStatPresent = 1` with a `pad` that cannot be confused with anything
- `250` on the snails of map 40, whose real `PADamage` is `3`. Every other field from the mob's
own WZ row, so exactly one input moves.

| on screen | reading |
|---|---|
| the client's own number becomes **~250-350** | **Reading (1).** The attack power was the variable, the server can drive the client's own number, and the two-number problem is over: send the mob's real attack power, delete our `0x02D1`, and the client draws the one correct number by itself |
| still **`1`**, and `world.log` shows the `0x03C6` went out at 137 + 57 bytes | **Reading (2).** The hit-type gate is zeroing it. Next stop is the **player's** stat block - `FUN_140268140`'s `param_5+0x10` and `+0x1c` - not the mob |
| mobs stop spawning, or the map is empty | the 57 bytes desynchronised the body. Compare the `0x03C6` length in `world.log` against `MOB_ENTER_FIELD_LEN + MOB_FORCED_STAT_LEN`; the block is all-or-nothing |
| mobs spawn but never attack, or the HP bar is wrong | the block overrode `level`/`acc`/`max_hp` with something the template disagreed with. §5.1 - it is not a merge |

The **discriminator is the number itself**, and both readings are already written down, so the
run is a measurement rather than a look. Neither outcome is a wasted launch: reading (2) is as
informative as reading (1) and eliminates the mob half of the search space entirely.

**Do not also remove our `0x02D1` on this run.** Two numbers is the current state and it is
readable; changing two things at once has produced one unexplained crash on this project
already. Ours stays until the client's own number is confirmed correct.

---

## WIRE IT LIKE THIS

`crates/net/src/mobdamage.rs` is new and complete (`cargo test -p net`: 464 pass; clippy clean).
`crates/net/src/lib.rs` has `pub mod mobdamage;`. **Nothing else is wired** - `crates/net/src/mob.rs`
is not mine to edit, and this is the "built is not wired" state `CLAUDE.md` asks to be loud about.

In `crates/net/src/mob.rs`, `mob_enter_field`, replace the one line

```rust
b.push(0); //                                         10   u8  141d33734 forced stat: NO
```

with

```rust
match mob.forced_stat {
    None => b.push(0), //                             10   u8  141d33734 forced stat: NO
    Some(fs) => {
        b.push(net::mobdamage::FORCED_STAT_PRESENT);
        b.extend_from_slice(&fs.encode()); //          +57 bytes, FUN_14085acd0
    }
}
```

and add `pub forced_stat: Option<MobForcedStat>` to `FieldMob` (default `None`, which keeps
today's bytes identical). `FieldMob::body_len` must add `MOB_FORCED_STAT_LEN` when it is
`Some` - the block is inside the body, and a wrong length desynchronises everything after it.

Build the value from the mob's own WZ row so only `pad` moves:

```rust
// gm-handbook/mobtemplates.txt: templateId, maxHP, maxMP, level, exp, PADamage, PDDamage,
//                               MADamage, MDDamage, acc, eva, ...
let mut fs = MobForcedStat::from_template(
    max_hp, level, pa_damage, ma_damage, pd_damage, md_damage, acc, eva,
);
fs.pad = 250; // the experiment; normally leave it at pa_damage
```

Two things a first attempt gets wrong, both from §5.1:

* **It is not a merge.** Leaving `level` or `acc` at zero changes the hit-type calculation as
  well as the damage. Always go through `from_template`.
* **`speed` (wire 10) is the only field where `0` means "leave the template's value alone"** -
  `FUN_14046ad90` guards it with `if (0 < ...)`. Every other zero is a real zero.

---

## 8. What I did **not** establish

* **Which link delivers the zero** (§6). The two candidates need opposite fixes and §7 decides
  between them in one launch.
* **The path from `FUN_140265f00`'s output to `FUN_1428aa0a0`'s `[rbp-0x60]`** - whether the
  reported damage is `param_2[1]` (type-gated, can be 0) or `param_2[2]` (always >= 1). This is
  what makes §6 two readings instead of one.
* **What `FUN_140268140` returns in practice**, and what the player-stat fields at `+0x10` and
  `+0x1c` are on the wire. That is the whole of reading (2) and it is unopened.
* **Whether the client would still draw its own number if we sent the right attack power.** It
  would - that is the official behaviour and the point - but nothing here measures it.
* **What `0x0313` and `0x0314` actually are.** Both are server-triggered self-damage that the
  client computes and reports with attack index `-11`; neither reads a packet field for the
  amount. I did not identify the trigger.
* **`0x02F2` and `0x02F3`.** Both reach the renderer and both are ungated; `0x02F2` reads one
  `u32` and goes into COM/VARIANT script machinery, `0x02F3` negates a value from `[rsp+0xc8]`.
  Neither is a mob hit, which is all I needed of them.
* **Whether `FUN_141866780` can make `param_3` something other than 100** (§4.2), which would
  scale every mob's attack power.
* **`forcedStat + 0x50`**, read by the consumer and never written by the parser (§5.1).

---

## 9. Reproducing every number here

Repo root as the working directory, always.

```bash
python tools/callers.py  0x1402fa9a0        # control: 96 sites, 43 in 0x140304b20
python tools/callers.py  0x142771360        # 30 sites / 23 fns / 0 tail jmp / 0 data ptr
python tools/callers.py  0x14046ad90        # 4 call sites - the stat initialiser
python tools/callers.py  0x14047a190        # 1 call + 2 TAIL JMP  <- corrects suppress.md
python tools/callers.py  0x141cc9410        # 4 sites in 2 fns

python tools/listing.py  0x14085acd0        # the forced-stat parser, 57 body bytes
python tools/listing.py  0x141c76190        # 141c76241 mov eax,0x64 - the pct is NOT byte 5
python tools/listing.py  0x141d33630        # 141d3373b je - forcedStatPresent 0 skips the parse
python tools/listing.py  0x141c4ff80 | grep -B26 141c53418
python tools/dump_va.py  0x14289d660 872    # the local-user jump table, 218 entries
python tools/dataref.py  0x143a48078        # the PADamage slot -> 0x140480a89 in FUN_14047d990
python tools/dataref.py  0x143a48050        # control: the maxHP slot

cargo test -p net
```

Decompilation produced for this file (Ghidra, JDK 21, `research/ghidra` project `msexe`):

| file | functions |
|---|---|
| `research/msexe-damagecalc.txt` | `140265f00`, `1402661b0`, `140499fa0`, `1401ba9d0`, `140268a60`, `1428ee7d0`, `1428a3bf0`, `142903600` |
| `research/msexe-mobattackpower.txt` | `14025e540`, `14025e610`, `141c54e50`, `141d15f60`, `140268140`, `1407e3ab0`, `141c54dd0` |
| `research/msexe-mobstat-object.txt` | `141c4cee0`, `1404751c0`, `140495990`, `14046ad90` |

The property-name strings are **UTF-16** and are reached through pointer slots in `.data` at
`0x143a48000+`, not by `lea` against the string - which is why `tools/dataref.py` must be aimed
at the slot and not at the text. §1.1.
