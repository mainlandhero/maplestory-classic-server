# Two damage numbers: the "stub" framing was wrong, and the owner was right to reject it

**2026-08-28.** The owner: *"You claim that the 1 cannot be turned off by packet, but the fact that
this is the official client and this behavior does not exist (showing 2 damage numbers by mobs)
means that something is wrong with our implementation."*

They are right, and this file retracts two things I said today.

Markers: **[L]** read off this client's listing, its WZ or a capture, **[D]** derived from two or
more [L] facts, **[I]** inferred.

---

## 0. The two retractions

**"The 1 cannot be turned off by packet."** Too strong. What is measured is narrower: *the
`/hitdamagetest` console command was refused by its permission gate*, which is one route
failing, not the absence of a cause. I generalised one failed lever into a property of the
problem.

**"The client draws its own stub damage number."** This framing came from earlier work and I
carried it forward without checking it. It is wrong in a way that matters: **the block that
draws the `1` is not a debug path that we accidentally enabled. On this client it is the only
path there is.**

---

## 1. Both halves of the gate are constants

`FUN_14288ac30` is the `0x00E5` builder, and at `0x14288b410`:

```asm
14288b400  test r13, r13
14288b403  je   0x14288b40b
14288b405  mov  edi, [r13 + 0x50]      ; from the ATTACK record
14288b409  jmp  0x14288b40e
14288b40b  mov  edi, r14d              ; ...or ZERO, when there is no attack record
14288b40e  xor  edx, edx
14288b410  mov  ecx, 0xae
14288b415  call 0x14090d160            ; GetOption(0xAE, 0)
14288b41a  test eax, eax
14288b41c  jne  0x14288b483            ; option non-zero -> the other branch
14288b41e  cmp  byte ptr [r15 + 0x544a], al     ; al == 0 here
14288b425  je   0x14288b483            ; flag zero      -> the other branch
           ; ---- otherwise this block runs ----
14288b439  mov  [rbp+0x80], edi        ; edi goes into the struct
14288b456  mov  edx, r12d              ; the damage so far
14288b459  mov  ecx, 1
14288b45e  call 0x140266ea0            ; -> damage
14288b463  mov  r12d, eax
```

### 1.1 Option `0xAE` is never written. **[L]**

Byte-scanned the whole of `.text` for `mov ecx, 0xAE` and `mov edx, 0xAE` and resolved the call
that follows each:

```text
37 sites load 0xAE into ecx/edx
  11 of them are  mov ecx,0xae ; call 0x14090d160   <- GetOption. ALL READS.
   0 of them write it.
  the rest load edx for unrelated callees (0x1402bf6d0, 0x1406ed520, 0x1418060c0, ...)
```

**Positive control**: the two sites already known from `damage-number-suppress.md` -
`0x1428ac8a6` and `0x14288b410` - are both in the result. The scan can find what it is looking
for.

`GetOption` returns its `edx` argument on a miss, and every call site passes `edx = 0`. So
**`GetOption(0xAE, 0)` is 0 for the life of the process**, and the `jne` never taken.

This closes `damage-number-suppress.md` §8's open question - *"whether key `0xAE` is a second,
independent lever"*. **It is not a lever at all.** Nothing in the client can set it, so no
packet can either.

### 1.2 `user+0x544a` is a constant too

`0x142883687  mov qword ptr [rsi+0x5448], 0x10001` - little-endian, that puts **`01` at
`+0x544a`**. It is an immediate, not a computed value. **[L]**

Whether that store runs for *this* user object is **[D]**, not [L]: `damage-number-suppress.md`
§8 lists *"whether `FUN_142882250` constructs the local user specifically"* as unresolved. But
the observation settles it from the other end - the number is on screen, so the byte is set.

### 1.3 Therefore

**Both conditions are constants, so the block always runs.** [D] It is not a mode. Calling its
output a "stub" was a description of one observation - every value seen was 1 - dressed up as a
mechanism.

And `/hitdamagetest 0` would not have "turned the number off". It would have sent the code down
the **other** branch, `0x14288b483`, which multiplies the damage by a factor from
`FUN_1408843e0`. That might well be the better branch. Nobody established that it is, and the
command was refused before it could be found out.

---

## 2. So why is it 1

The block's own input, four instructions earlier: `edi = [r13 + 0x50]`, **or `0` when `r13` is
null**. `edi` is written into the struct at `[rbp+0x80]` that `FUN_140266ea0` receives.

`r13` is the mob's **attack record**. `research/touch-damage.md` §3 established **[L]** that
*the snail has no attack node* - `FUN_141c69f40` walks `mob+0x9d8`, the list of active attacks,
and a body/touch attack has none.

Enumerated over the client's own `Mob_000.wz`, all 193 mob images:

```text
 65 mobs HAVE an attack node   (35, 38, 41, 50, 52, 53, 61, 63, 1003, 1004, 1005, 1019, ...)
128 mobs do NOT                (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, ...)
```

**Every mob the owner has ever fought is in the 128.** Map 40 is Snail Hunting Ground I; templates 1,
2 and 3 all have no attack node. The 224 captures behind *"mobs rated 3 to 287, all 1"* are
therefore **224 observations of the same case**, not a sweep across the range - the ratings vary,
the code path does not.

**[D]**: with no attack record the struct field is 0, and `FUN_140266ea0` - which begins
`test edx,edx / jle -> return` - produces the floor.

---

## 3. The experiment, and it needs one mob nobody has fought

**Fight a mob that HAS an attack node.** It discriminates in one hit and needs no new code:

| outcome | reading |
|---|---|
| a **real** number, matching the mob's `PADamage` | the missing attack record is the cause. Body-attack damage is what the server owes the client, and the fix is on the touch path |
| still **1** | the attack record is not the variable, and §2 is wrong. Then the struct's other fields, or `FUN_140266ea0` itself, is where to look |
| **no** number from us, one from the client | we should stop drawing ours |

Reachable maps, from `gm-handbook/mobs.txt` joined to the attack-node list:

| map | name | template | level | PADamage |
|---|---|---|---|---|
| **1014** | Snail Field of Flowers | 41 | 46 | 243 |
| 41 | Snail Hunting Ground II | 53 | 75 | 410 |
| 10001012 | Henesys Hunting Ground III | 50 | 60 | 306 |

Map 1014 is the mildest of them and still hits far harder than anything on map 40. That is the
point: **1 versus 243 is not a number anyone has to squint at.**

---

## 4. The second number is ours, and that is the other half of the owner's point

On a real server there is **one** number because the server does not draw one. The client draws
what it computed; the server sends only the consequence.

We draw a second, through `0x02D1` effect `0x41` with a negative value - and ours is *correct*,
because `world::damage::incoming_damage` runs the real formula over the mob's `PADamage` and the
player's defence. So the screen currently shows the client's wrong number and our right one.

**Removing ours is one line** and would leave exactly the official behaviour - one number, drawn
by the client, currently saying 1. It is not done here, because until §3 runs, ours is the only
number on screen that is right, and deleting the right number to match a wrong one is the wrong
order to do this in.
