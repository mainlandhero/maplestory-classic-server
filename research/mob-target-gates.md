# Why the client collects zero targets - the gate chain, re-read

> ## CORRECTION, 2026-08-20: §6 answers about a caller the client does not use
>
> Measured, off `research/fixtures/melee-collector-runs-once-per-swing-hook.log`: all six
> entries to `FUN_141d31b20` carry **`called-from=0x141d2545a`** - that is `FUN_141d25360`,
> calling at `141d25455`. **Not `0x1428c2c2d`.** [L]
>
> §6 rules out gates 3, 5, 11, 14, 15 and 16 because they are *"switched off by the arguments
> at the only known call site"*, and that call site is `0x1428c2c2d` inside `FUN_1428c1fa0`.
> A different arm passes different arguments, so **those six gates are not ruled out for the
> path the client actually takes.** The rest of §6 - the gates that are WZ properties or
> constructor values - is unaffected.
>
> §4.2 also needs re-reading: `141d31c96 cmp/jge` is the **loop header**, not a one-shot
> precondition. Argument 4 is the loop's capacity and argument 17 is the output cursor,
> incremented at `141d327de`/`141d32939` with `141d32a62` re-entering. On the six measured
> entries `r9 = 0xf`, so the capacity is **15**, the cursor is 0, and **the loop ran**. [L]
>
> The call-site table is `research/mob-collector-callsites.md`; the arm-C gate analysis
> replaces §4 and §6 for this question.

Written 2026-08-19 to answer one question: **the client will not target our mobs. Why?**

Markers as elsewhere in `research/`: **[L]** read out of the listing or a capture, **[D]**
derived from two or more [L], **[I]** inferred.

**No Ghidra** - another agent holds the project lock. Everything here is capstone over
`client-patched\MapleStory.exe` via `tools/reads.py`, `tools/callers.py`,
`tools/fieldrefs.py`, plus scratch instruments described in section 8, plus the capture
`research/fixtures/attack-with-mobs-present-zero-targets-world.log`.

The full listing of the collection function is preserved beside this file as
**`research/msexe-mobtarget-collect.txt`** (`0x141d31b20 .. 0x141d32b1c`, 1038 lines).

---

## 0. Answer up front

| | |
|---|---|
| **the `move_action` lead is dead** | gates 2 and 8 are **not** written from the `move_action` jump table. They are written from the **`appear_type`** switch, and the arm our `appear_type = -2` takes cannot reach either writer. Proved by forward CFG reachability plus a field scan with its positive control. §2 |
| **the byte that *could* do it is `appear_type`** | `appear_type >= 0` takes the switch **default**, which sets `mob+0x504 = 1` - and gate 2 rejects any mob with `mob+0x504 != 0`. We send `-2`, which is safe. **Do not "fix" the spawn by sending the WZ's `summonType` (1).** §2.3 |
| **the gates are eleven only if you stop reading** | **27** branches inside the loop body jump past the accept, at **three** labels. §3 |
| **every gate our packets can touch, passes** | gate by gate in §4. Gate 1 is set to 1 on the new-mob branch *before* `encodeInit`; gates 2 and 8 keep their constructor values; gates 9, 10 and 13 are WZ properties (`allyMob`, `notDamaged`, `onlyHittedByCommonAttack`) a snail does not have |
| **our mobs are in the list the collector walks** | `pool+0x58` is the head of the very list `0x03C6` inserts into. Derived from the insert helper's own stores, and the iterator arithmetic matches the walk instruction for instruction. §5 |
| **so the first gate our mobs fail is: none of them** | static analysis cannot find a per-mob filter that rejects a snail. That is a negative result, and it moves the question upstream. §6 |
| **and upstream there is a real structural doubt** | of the **six** builders of `0x00DF`/`0x00E0`/`0x00E1`, **only `FUN_1428c1fa0` calls `FUN_141d31b20` at all** - and it has two other collectors beside it. It is **not established** that this function runs for a plain "User Melee" swing. §6 |
| **an early-out nobody had recorded** | `141d31c96 cmp rax,rcx / jge` returns **before touching a single mob** when argument 17 >= argument 4. And the return value is `(accepted << 16) \| arg17`, so `1428c2c32 mov [rbp+0x870],eax` is **a packed pair, not a count**. §4.2 |

---

## 1. The measurement, tightened: they were standing on top of one

`STATUS.md` and the task both rest on the fixture
`research/fixtures/attack-with-mobs-present-zero-targets-world.log`: 34 spawns, 1396 move
reports, 5 `0x00DF`, all 127 bytes. The obvious alternative - *they swung at nothing* - has now
been closed **inside that one file**, which is the standard `CLAUDE.md` sets. [L]

Every `0x00DF` decodes field for field against `research/mob-combat.md` §1.3 with
`targetCount = 0` and nothing left over, and every `0x02FF` carries the client's own idea of
where each mob is. Pairing them:

| attack | player | nearest mob, last reported before the swing | gap | targets |
|---|---|---|---|---:|
| 02:25:35.692 | (741, 395) | 2003 at (782, 395), 0.32 s earlier | 41 px | **0** |
| 02:25:37.031 | (794, 395) | 2003 at (830, 395), 0.58 s earlier | 36 px | **0** |
| **02:25:38.251** | **(873, 395)** | **2003 at (877, 395), 0.73 s earlier** | **4 px** | **0** |
| 02:25:39.962 | (1028, 395) | 2004 at (1062, 395), 0.28 s earlier | 34 px | **0** |
| 02:25:52.054 | (1473, -205) | 2016 at (1428, -205), 0.49 s earlier | 45 px | **0** |

Same ground line, same session, both halves from the same log. **Four pixels.** [L]

Two details worth keeping. The mob positions are the client's own (`0x02FF`), not ours - the
spawn put 2003 at `(763, -385)` and the client walked it to `(877, 395)`, so the client is
simulating them on real footholds. And the attack body's own `x, y` (fields 13/14) is the
player's position, which is how the pairing is possible at all.

> Decoder: `where.py` in the session scratchpad; it reuses `tools/decode_mobmove.py`'s layout,
> which that tool proves against body length on every one of the 1396 bodies.

---

## 2. `move_action` cannot reach gates 2 or 8 - and `appear_type` can

`research/mob-combat.md` §11.2 says gates 2 (`mob+0x504`) and 8 (`mob+0x300 == 0x38`) are
"both written in the `move_action` jump-table region (`141c52a95..141c52b7d`)". **The region is
real and the attribution is wrong.** That region is the *default arm of the `appear_type`
switch*. [L]

### 2.1 Where the `move_action` table actually is, and what it does

```asm
141c50dab  call 0x1409c6d00        ; eax = the move_action byte, deobfuscated
141c50dba  mov  edi, eax
141c50dbc  and  edi, 1             ; facing
141c50dbf  sar  eax, 1             ; action
141c50dc1  dec  eax                ; action - 1
141c50dd0  cmp  eax, 0xf
141c50dd3  ja   0x141c50e20        ; default
141c50dd7  jump table at 0x141c53fe4, 16 entries
```

Every arm does one thing: put a **stance code** in `edx` and fall into
`141c50e34 mov rcx,rsi / call rbx`, where `rbx = [mobvtbl + 0xd0] = FUN_141cbcad0`. [L]

| `move_action` | action | table index | arm | stance passed |
|---:|---:|---:|---|---|
| 2, 3 | 1 | 0 | `141c50de3` | `0x4e` or `0` (from `FUN_141d15b20(mob+0xbd8)`) |
| 4, 5 | 2 | 1 | `141c50df8` | 1 |
| 6, 7 | 3 | 2 | `141c50dff` | 2 |
| 12, 13 | 6 | 5 | `141c50e06` | 3 |
| 16, 17 | 8 | 7 | `141c50e12` | 4 |
| 26, 27 | 13 | 12 | `141c50e19` | `0x43` |
| 32, 33 | 16 | 15 | `141c50e0b` | `0x2f` |
| 8-11, 14, 15, 18-25, 28-31, and 0, 1, and >= 34 | 4, 5, 7, 9-12, 14, 15, and 0 and >= 17 | 3, 4, 6, 8-11, 13, 14, and out of range | `141c50e20` | 1, or 3 when `template+0x74 == 3` |

(Action `0` reaches the default arm too: `dec eax` makes it `0xffffffff` and the unsigned
`cmp eax,0xf / ja` is taken. That is a separate matter from the `move_action 0` crash, which
happens earlier, at the `141c50da5` callback - `research/mob-spawn.md` §11.)

`FUN_141cbcad0` writes **neither** `mob+0x300` nor `mob+0x504`: a `tools/fieldrefs.py --write`
scan over the whole mob code range gives exactly **two** writers of `+0x300` (the constructor's
`0xffffffff` at `141c4d26e` and `141c52b44`) and **thirteen** of `+0x504`, none of them inside
`0x141cbcad0..0x141cbcd2d`. The scan's documented positive control (`+0x2f4`, three rows) was
reproduced first. [L]

The result comes back as `eax` and is folded into the mob's own obfuscated action word:
`141c50e43 lea ebx,[rax+rax] / or ebx,edi` then the three-word stash at
`mob+0x3dc/0x3e0/0x3e4`. So `move_action` round-trips into the animation state and **nothing
else**. [D]

### 2.2 Where gates 2 and 8 are really written

```asm
141c51a72  mov  r15d, dword ptr [rbp + 0x70]   ; [rbp+0x70] := appear_type at 141c5048d
141c51a76  lea  eax, [r15 + 6]
141c51a81  cmp  eax, 5
141c51a84  ja   0x141c52a95                    ; DEFAULT
141c51a8c  jump table at 0x141c54024, 6 entries
```

and in the default arm:

```asm
141c52b36  mov   edi, 1
141c52b44  mov   dword ptr [rsi + 0x300], r15d
141c52b4b  mov   dword ptr [rsi + 0x504], edi   ; = 1  ->  GATE 2 REJECTS
```

`[rbp+0x70]` is written once before this point, at `141c5048d`, immediately after the
`appear_type` read at `141c50485`. [L]

### 2.3 Which `appear_type` values are safe - measured by CFG reachability

Forward reachability inside `FUN_141c4ff80`'s merged extent (`0x141c4ff80..0x141c54054`) from
each arm, to the five writers of the two gate fields. `reach.py` follows both edges of every
conditional, stops at `ret`/`int3`, and reports indirect jumps rather than guessing them; from
each arm below it encountered none. [L]

| `appear_type` | arm | writes `mob+0x504` | writes `mob+0x300` | verdict |
|---:|---|---|---|---|
| -6 | `141c52772` | no | no | targetable |
| -5 | `141c5274e` | **yes** `141c52764` | no | **gate 2 depends on the value stored** |
| -4 | `141c523bf` | **yes** `141c52708` | no | ditto |
| -3 | `141c5236b` | **yes** `141c523a9` | no | ditto |
| **-2 (ours)** | `141c51d59` | **no** | **no** | **both fields keep the constructor's values** |
| -1 | `141c51a99` | no | no | targetable |
| **>= 0** | default `141c52a95` | **yes, and the value is the literal 1** | yes | **untargetable, gate 2** |

Constructor values: `141c4d5eb mov [rsi+0x504], r14d` and `141c4d26e mov [rsi+0x300],
0xffffffff`. `r14d` is the constructor's zero register - `141c4cf2b xor r14d,r14d`, and it is
never reloaded before `141c4d5eb`; it is what zeroes the whole `+0x100..+0x508` run. So
`mob+0x504 = 0` and `mob+0x300 = -1`, and `-1 != 0x38`, so gate 8 passes. [L]

> **The one-line summary of §2.** With `appear_type = -2`, **no value of `move_action`
> changes gate 2 or gate 8**, and both pass. `move_action` is exonerated. `appear_type` is the
> byte that steers them, and `>= 0` is the value that would break targeting - which is exactly
> what `research/mob-spawn.md` §2f floats when it notes the WZ's `summonType` is `1` for both
> reachable templates. **Sending `summonType` would make every mob permanently unhittable.**

---

## 3. There are 27 rejections, not eleven, at three labels

The loop body's **accept** is the count increment at the very bottom:

```asm
141d329f0  mov   edi, dword ptr [rsp + 0x38]
141d329f4  lea   eax, [rdi + 1]
141d329f7  test  sil, sil
141d329fa  cmove eax, edi              ; sil == 0 -> no increment
141d329ff  mov   dword ptr [rsp + 0x38], eax
```

Anything that jumps past `141d329f4` is a rejection. Enumerated rather than eyeballed
(`rejects.py`, over the whole merged extent): [L]

| label | branches | what they are |
|---|---:|---|
| `141d32a18` | 3 | inside a nested container-growth block, plus two list-walk exits |
| `141d32a1d` | 10 | gates 15-17 plus seven inside a template-driven list block (§4.3) |
| `141d32a24` | 14 | gates 1-14 |
| **total** | **27** | |

(`141d32a06 jne 0x141d32a18` is **not** one of them - it sits after the increment.)

`research/mob-combat.md` §11 lists eleven, and those eleven are correct as far as they go: they
are the first eleven of the fourteen at `141d32a24`. Gates 12, 13 and 14 and the ten at
`141d32a1d` were below where that pass stopped reading.

---

## 4. Gate by gate: what it reads, what passes, who can set it

`rbx` is the list node, `[rbx+8]` is the mob. Every `call 0x142e52ed0` with `ecx = 0x431` is
the null assertion and is noise. Argument numbering follows `research/mob-combat.md` §11.1 and
is re-derived below in §4.1.

| # | at | predicate | passes when | set by | can the server set it? |
|---:|---|---|---|---|---|
| 1 | `141d31cbf` | `FUN_141c543c0(mob)` - the obfuscated flag at `mob+0x2d8` | **non-zero** | `FUN_141c543e0(mob,1)` at `141d338f2` on `0x03C6`'s **new-mob** branch, *before* `encodeInit` at `141d33929`; `0x03D1` sets it to 0 | no field on the wire. **Sending `0x03C6` is what sets it.** Verified: only three writers of `+0x2d8` in the mob code, and one is the constructor |
| 2 | `141d31ce5` | `mob+0x504 != 0` | **zero** | ctor 0; `encodeInit` only in the `appear_type` arms of §2.3; four packet handlers (`0x03D9..0x044D` family) set 2 and 3 | **yes, via `appear_type`** - and `-2` already passes |
| 3 | `141d31d21` | arg5 non-null: compare `mob+0x3a0` (object id) | ids **differ** | the packet's object id | arg5 is **0** at the one known call site, so off |
| 4 | `141d31d53` | arg6 non-zero: `mob+0x3a0` | **equals arg6** | | arg6 is `[rbp+0x6f80]`, a client field. If non-zero this is a **single-target** filter |
| 5 | `141d31d84` | arg8 non-zero: `template+0x60` | **equals arg8** | the template id | arg8 is **0**, off |
| 6 | `141d31dbd` | skill node non-null: `FUN_1407a2eb0(node, templateId)` | **true** | `FUN_1407b2910(arg14)` at `141d31b8e` | arg14 is a skill id. With no skill the node is null and the gate is skipped |
| 7 | `141d31dec` | `template+0x130 > 0` **and** arg10 == 0 | `template+0x130 == 0` | WZ; name is a runtime-decrypted string (`mov rdx,[rip]->143a489e0`) | template-only |
| 8 | `141d31e12` | `mob+0x300 == 0x38` | **not 0x38** | ctor `0xffffffff`; `encodeInit` only in the default `appear_type` arm | **yes, via `appear_type`** - `-2` passes |
| 9 | `141d31e40` | arg9 == 0: `FUN_141c565b0` = `[mob+0x3c8]+0x1e0 != 0 \|\| template+0x138 != 0` | **both zero** | `template+0x138` is the WZ property **`allyMob`** (`lea rdx -> u"allyMob"` at `140481ce8`) | template-only; a snail is not an ally mob |
| 10 | `141d31e68` | `FUN_141c561a0(mob,0)` | **returns 0** | tests `template+0x60 == 0x7dbc16` (8240150, a boss), `[mob+0x3c8]+0x404`, `+0x2c0`, and `template+0x150` = WZ **`notDamaged`** (`"notDamaged"` at `1404820a6`) | template-only |
| 11 | `141d31ed0` | if `arg11 == mob+0xb50` **and** arg12 != 0: `abs(arg13 - iface->[+0x30]()) <= arg12` | | | arg12 is **0** at the known call site, so the distance test never runs |
| 12 | `141d320ca` | `sil == 0`, where `sil` is set only when `[skillNode+0x110] != 0` and one of two sub-tests holds | | skill data | skipped with no skill node |
| 13 | `141d320f0` | `FUN_141c55c80` = `template+0x190` | **zero** | WZ **`onlyHittedByCommonAttack`** (`u"onlyHittedByCommonAttack"` at `140481de5`) | template-only; snails do not have it |
| 14 | `141d3211e` | arg15 != 0: `FUN_141c54f00` = `template+0x81` | **non-zero** | WZ, runtime-decrypted name; §12.2 of `mob-combat.md` ties the same byte to owning an HP gauge | arg15 is **0** at the known call site, so off |
| 15 | `141d32154` | arg16 non-null and `[arg16+4] != 0`: `template+0x81` | **zero** - the inverse of 14 | | off unless arg16 is passed |
| 16 | `141d32177` | `[arg16+0x70]` non-null: its `vtbl+0x10(mob)` | **non-zero** | | off unless arg16 is passed |
| 17 | `141d321ae` | arg14 == `0x231c49` (skill **2301001**): `template+0xc0` | **non-zero** | WZ, decrypted name | one specific skill only |
| 18-24 | `141d322ab`-`141d32409` | a block entered when `FUN_14047c1d0(template)` returns a non-empty list at `141d321df`; involves `FUN_140479f80`, `FUN_140499c50`, and an object-id compare | | **not resolved** | **not resolved** - see §7 |

### 4.1 The argument map, re-derived and extended

Seven pushes then `lea rbp,[rsp-0x560]` put argument *N* at `[rbp + 0x5A0 + 8(N-1)]`. That
reproduces `research/mob-combat.md` §11.1 exactly for arguments 5-14 and extends it: [D]

| slot | arg | slot | arg |
|---|---|---|---|
| `rbp+0x5c0` | 5 | `rbp+0x600` | 13 |
| `rbp+0x5c8` | 6 | `rbp+0x608` | 14 |
| `rbp+0x5d0` | 7 | `rbp+0x610` | **15** |
| `rbp+0x5d8` | 8 | `rbp+0x618` | **16** |
| `rbp+0x5e0` | 9 | `rbp+0x620` | **17** |
| `rbp+0x5e8` | 10 | `rbp+0x628` | **18** |
| `rbp+0x5f0` | 11 | | |
| `rbp+0x5f8` | 12 | | |

At the one call site that uses this collector, `1428c2c2d` in `FUN_1428c1fa0`, the caller
stores `ebx = 0` into every stack slot except two: [L]

```asm
1428c2be9  mov [rsp+0x68], r15d      ; arg14 - a skill id
1428c2c0a  mov eax,[rbp+0x6f80] / mov [rsp+0x28], eax   ; arg6 - a single object id
1428c2c19  mov r9d, esi              ; arg4
1428c2c1c  lea r8,  [rbp+0x6d20]     ; arg3
1428c2c23  lea rdx, [rbp+0x520]      ; arg2 - the rect, four dwords
1428c2c2a  mov rcx, r13              ; arg1 - the pool
```

So arguments 5, 7-13, 15-18 are all zero here, which turns off gates 3, 5, 11, 14, 15 and 16
and leaves gate 4 live only if `[rbp+0x6f80]` is non-zero.

### 4.2 The early-out, and the return value that is not a count

```asm
141d31c76  movsxd rax, dword ptr [rbp + 0x620]   ; argument 17
141d31c96  cmp    rax, rcx                        ; rcx = argument 4
141d31c99  jge    0x141d32a67                     ; RETURN, nothing walked
...
141d32a67  mov ebx,[rsp+0x38] / shl ebx,0x10 / or ebx,[rbp+0x620]
141d32aae  mov eax, ebx
```

Two consequences, both new: [L]

1. **If argument 4 <= argument 17 the loop never starts** and no mob is examined. Argument 17
   is 0 at this call site, so `arg4 <= 0` returns zero targets before a single gate runs.
   Argument 4 is `esi` in `FUN_1428c1fa0` - a client-side maximum, not anything we send.
2. **`eax` is `(accepted << 16) | arg17`.** `1428c2c32 mov [rbp+0x870], eax` therefore stores a
   packed pair, and `research/mob-combat.md` §11's label "THE TARGET COUNT" is at best the high
   half. Anything that reads that slot as a plain integer is reading two fields.

### 4.3 The four helper predicates, verbatim

```asm
FUN_141c55a80:  xor eax,eax / cmp [rcx+0x504],eax / setne al / ret        ; gate 2
FUN_141caeaf0:  xor eax,eax / cmp [rcx+0x300],0x38 / sete al / ret        ; gate 8
FUN_141c55b00:  mov rdx,[rcx+0x3a8] / cmp [rdx+0x130],0 / setg al / ret   ; gate 7
FUN_141c55c80:  mov rax,[rcx+0x3a8] / movzx eax,byte [rax+0x190] / ret    ; gate 13
FUN_141c54f00:  mov rax,[rcx+0x3a8] / movzx eax,byte [rax+0x81] / ret     ; gates 14, 15
```

---

## 5. Our mobs are in the list the collector walks

Worth settling because "the walk sees a different container" would have explained everything.

The collector walks `[arg1 + 0x58]` and steps with
`mov rdx,[rbx-0x20] / lea rax,[rcx+0x28] / cmovne rbx,rax`, taking the mob from `[rbx+8]`.

`0x03C6`'s new-mob branch inserts with `FUN_141d4edf0(pool+0x38, &mob)` at `141d337e1`, and
that function's own stores give the layout: [L]

```asm
141d4ee46  mov [rbx+0x30], rax        ; node+0x30 := the mob
141d4eeb4  lea rax, [rbx+0x28]        ; the iterator value is node+0x28
141d4eebd  mov [rsi+0x20], rax        ; empty list: head := it     (rsi = pool+0x38)
141d4eec1  mov [rsi+0x28], rax        ;             tail := it
141d4eeee  mov [r14-0x20], rbp        ; otherwise link the old tail forward
```

`rsi = pool + 0x38`, so **head = `pool + 0x58`** and tail = `pool + 0x60`. From an iterator
`it = node + 0x28`: the mob is at `it + 8` = `node + 0x30`, and the forward link is at
`it - 0x20` = `node + 0x08`, with the next iterator at `+0x28` into the next node. That is the
collector's walk instruction for instruction. **[D]**

(The by-object-id lookup `FUN_141d2efc0` uses a different structure, the hash at `pool+0x68`
with the bucket count at `pool+0x70` - which is why the client can find a mob for
`0x03D2`/`0x03E4` regardless.)

---

## 6. So which gate do our mobs fail? None that static analysis can find

Putting §2, §4 and §5 together, for a snail spawned by our `0x03C6` and granted by our
`0x03D2`:

* gate 1 passes - `0x03C6`'s new-mob branch sets it to 1 before `encodeInit` runs
* gate 2 passes - `mob+0x504` is the constructor's zero, for every `move_action`
* gate 8 passes - `mob+0x300` is the constructor's `0xffffffff`
* gates 9, 10, 13 pass - `allyMob`, `notDamaged`, `onlyHittedByCommonAttack` are absent from
  `Mob/0000001.img`, the same as they would be on a real server
* gates 3, 5, 11, 14, 15, 16 are switched off by the arguments at the only known call site
* gates 6, 12, 17 need a skill node, and a bare swing has none
* gate 7 is a template field we do not control and a real server does not either
* the mob is in the right list

**I am reporting that as a negative rather than picking a gate**, which is what the task asks
for when static analysis cannot single one out. It also moves the question, and the move is
the useful part:

### 6.1 It is not established that this function runs for a bare melee swing

`tools/callers.py 0x140f31fe0` gives the complete set of builders of the shared `0x00DF` /
`0x00E0` / `0x00E1` header - six functions. Cross-referenced against the 86 call sites of
`FUN_141d31b20`: [L]

| builder | opcode | calls `FUN_141d31b20`? | direct calls into mob code `0x141c40000..0x141d60000` |
|---|---|---|---:|
| `FUN_1428baa50` | `0x00DF` | **no** | **0** (12 indirect) |
| `FUN_1428c1fa0` | `0x00DF` | **yes**, `1428c2c2d` | **30** |
| `FUN_1429a6590` | `0x00DF` | no | 1 |
| `FUN_1428cd6d0` | `0x00E1` | no | - |
| `FUN_1429a7090` | `0x00E1` | no | - |
| `FUN_1429a7ab0` | `0x00E0` | no | - |

`FUN_1428baa50` touches no mob code at all - it serialises a list of up to 15 target blocks
built by somebody else (`142ef44fc(&[rbp+0x5c0], 0x1d8, 0xf, ctor, dtor)` at `1428bb314` is the
array constructor, and `[rbp+0x5b0] = rsi` immediately before it is the count). Its four
callers - `0x1418f5300`, `0x1419083c0`, `0x1419987a0`, `0x141f30aa0` - **also make no direct
call into mob code.** So if our capture came from `FUN_1428baa50`, the target list is built
somewhere nobody has read, and `FUN_141d31b20` never runs. [L]

And even inside `FUN_1428c1fa0` there are **three** collectors writing the same slot:

```
1428c2c2d  call 0x141d31b20   -> [rbp+0x870]
1428c30b1  call 0x141d2a4f0   -> [rbp+0x870]
1428c32e9  call 0x141d25360   -> [rbp+0x870]
```

`FUN_1428c1fa0` has **48 call sites in 37 functions**, the bulk of them in the per-skill band
`0x14297xxxx..0x1429axxxx`, which is what makes `research/mob-combat.md` §11.3 call this the
skill path.

**Attempts to settle it that did not work, recorded so they are not repeated:**

* `tools/xref.py --va 0x1432b3f10` ("User Melee") returns **0**, as its own docs predict - the
  string is not `lea`'d.
* A scan of all six builders' extents for any rip-relative operand landing anywhere in the
  attack-name table `0x1432b3ec0..0x1432b4700`: **zero hits in all six.** So the type name is
  put on the request object by whoever *builds* it, not by the encoder.
* The same scan over `FUN_1428baa50`'s four callers, `FUN_1429a6590`'s one caller, and the
  three group-B builders: **zero.**
* **A whole-image sweep for any rip-relative operand landing on `"User Normal"`,
  `"User Melee"`, `"User Shoot"` or `"User Magic"` finished with zero hits** - 666 939 resync
  points, i.e. it really did cover `.text` and `.boot`. Combined with `xref.py`'s zero, that
  says the attack-type name is reached by a **computed** address (table base plus an index),
  never by an instruction that names it. So no static search for the string will ever name the
  builder; the type enum has to be traced instead.

  **And the sweep was run against a positive control before that zero was believed.** The
  identical whole-image sweep for `0x14328b020` (`u"onlyHittedByCommonAttack"`) returns
  **exactly one hit**, `140481de5 lea rdx,[rip+0x2e09234] in 0x14047d990`, at the same
  **666 939** resync points:

  ```text
  python riprefs.py 0x14328b020
    140481de5  lea  rdx, [rip + 0x2e09234]  -> 14328b020  in 0x14047d990
    1 hit(s), 666939 resync point(s)
  ```

  Same code path, same coverage, one expected hit found and no others. So the zero for the
  four attack-type names is a property of the client, not of the search. **[L]**
* A second whole-image sweep, for the immediate `0x0834ae9f` (field 9 of the body,
  byte-identical in this capture and in the one `mob-combat.md` §1.3 quotes, so it is a
  constant rather than a nonce), **hit its 900 s timeout and produced nothing.** That is not a
  negative result and is not reported as one.

---

## 7. What is NOT established

1. **Which builder produced the captured `0x00DF`.** §6.1. This is the single most valuable
   open question and it decides whether §4's table is the right instrument at all.
2. **Whether `FUN_141d31b20` is entered when the owner swings.** No static argument can answer it;
   §9 is the test.
3. **Gates 18-24**, the block behind `FUN_14047c1d0(template)` returning a non-empty list at
   `141d321df`. Seven reject branches live in there and none of them was read.
4. **Argument 4 at the call site** - `esi` in `FUN_1428c1fa0`. If it is <= 0 the loop never
   runs (§4.2) and every gate is irrelevant.
5. **The WZ names of `template+0x81`, `+0xc0` and `+0x130`.** All three are loaded through
   `mov rdx, qword ptr [rip+...]` into a global holding a decrypted string, not through a
   `lea` of a literal, so `names.py` cannot resolve them the way it resolved `allyMob`,
   `notDamaged` and `onlyHittedByCommonAttack`. `research/msexe-strdecrypt.c` is the path if
   they ever matter.
6. **Touch damage.** Nothing here bears on it. Template 1 does carry `bodyAttack = 1` and
   `PADamage = 1` in `gm-handbook/mobtemplates.txt`, so a snail is meant to hurt on contact and
   for 1 point - which is small enough that "did no damage" and "did 1 damage" are hard to tell
   apart on screen. The cheap answer is still a capture.

---

## 8. Instrument notes

* **A scratch file called `dis.py` shadows the standard library.** `capstone`'s import chain
  pulls in the stdlib `dis`; a script of that name on `sys.path[0]` gets imported instead, and
  because it in turn imports `tools/reads.py`, the partially-initialised `capstone` makes
  `reads.py` raise its own `SystemExit("capstone is required: pip install capstone")`. Twenty
  minutes went into a tool that was "not installed" while `python -c "import capstone"` printed
  `5.0.7`. Do not name a scratch file after a stdlib module.
* **`tools/fieldrefs.py`'s positive control reproduces** - `0x2f4` over
  `0x141c40000..0x141d60000 --write` gives exactly `141c4d261`, `141c4e6ee`, `141cb7ef3`. Every
  `--write` count in this file was taken after checking it.
* **Whole-image Python sweeps are not a 15-minute instrument.** `.text` alone is 52 MB; a
  per-instruction operand scan over `.text` + `.boot` did not finish in 900 s. Bound the range
  or expect to wait.
* **`cut -c1-220` truncated a log line and made every `0x02FF` body look truncated**, which
  briefly looked like a finding about the logger. It was a property of the pager, not the file.
  The same class of mistake as everything else in this section.
* **An enumerated reject-branch scan beats reading down the function.** `rejects.py` finds all
  27 in a second; reading found eleven and stopped where the straight-line block ended.
* **Forward CFG reachability is the right tool for "can this arm reach that store".**
  `reach.py` answered in one call what a dominator argument had got wrong once already
  (`research/mob-spawn.md` §11.7).

---

## 9. The one-variant test

**Change nothing on the wire. Arm one probe watch, and swing.**

That is deliberate, and §2 is the reason: **no byte in the spawn packet has been found that
changes any gate in this chain.** Only two body bytes steer a gate at all - `appear_type`
(gates 2 and 8) and, through `0x03C6` being sent in the first place, gate 1 - and both already
sit on passing values. `move_action` is exonerated outright. The two other safe `appear_type`
values, `-1` and `-6`, reach the same field state by a different arm, so swapping to them
would test nothing about targeting. Spending a launch on a byte change that static analysis
says cannot matter is the mistake this project keeps paying for.

That is a claim about **this chain**, not about the body as a whole: 17 of the body's fields
are still sent as zero on a "no readable consumer" argument (`research/mob-spawn.md`), and
none of them was traced to a gate here either way.

**The change:** run with the probe watching the collection function's entry -

```
watch@141d31b20
```

**What the owner should do:** exactly what they did before. Enter the world on **map 40**, walk up
against a snail until they are standing on it, and swing **five or six times**. Nothing on screen
should look any different from the last run - the mobs spawn, move, and do not take damage.
**What they should watch for on screen is only that the client does not fault and that the
snails still move**; if either changes, the probe itself perturbed the run and the result is
void.

**The evidence is `client-patched\maplecw-hook.log`, and there are exactly three outcomes:**

| what the hook log shows | what it means | what to do next |
|---|---|---|
| **`WATCH 0x141d31b20 ENTERED` with `called-from=0x1428c2c2d`**, timed with the swings | `FUN_1428c1fa0` is the melee builder and the eleven-gate chain **is** the instrument. Since §4 says every gate our packets touch passes, the rejection is then in an argument, not in the mob - argument 4 (`esi`) or argument 6 (`[rbp+0x6f80]`) - or in gates 18-24 | watch `141d31c99` (the early-out) and `141d32a24`; the first tells you whether the loop ran at all |
| **`ENTERED` with a `called-from` that is NOT `0x1428c2c2d`**, or entries that do not line up with the swings | the function runs, but as the mob pool's own per-frame helper, and the melee path is elsewhere | treat the hits as noise and go to the row below |
| **no `WATCH` line at all while they are swinging** | **`FUN_141d31b20` is not the melee collector.** `research/mob-combat.md` §11 and §4 of this file are then the wrong function entirely, and the whole eleven-gate line of investigation is void | the next instrument is `FUN_1428baa50` (§6.1) - find what fills `[rbp+0x5b0]`, the count it serialises, by watching its four callers |

**One caution that decides whether the run is readable.** `FUN_141d31b20` has **86 call sites
in 74 functions**, and some of them are plausibly per-frame. If the log fills with `WATCH`
lines before the owner ever swings, that is the second row above, and the useful signal is the
`called-from=` value - which the hook records exactly. `research/mob-spawn.md` §2g is explicit
that the probe's `stack:` line is a heuristic scan and only `called-from=` can be trusted, so
**read `called-from=`, not the stack**.

### 9.1 If the run has to change a byte instead

Run **one of the two, never both.** The only byte with a mechanism behind it is the
**`0x03D2` controller level, `1` -> `2`** - `mob+0x960`, the aggro flag
(`FUN_141cc1e40` is `cmp byte [rcx+0x960],1 / seta al`).

* **On screen:** snails walk *toward* the owner and keep coming, instead of wandering.
* **They chase them** -> the client's mob AI has the player in its world and is testing mob
  against user every frame, so the mob objects are fully live and the failure is specific to
  the attack path. It also gives touch damage a chance to fire without them chasing anything.
* **They still wander** -> the mob AI does not see the player at all, which is a much more
  basic defect than a target filter and would redirect everything.
* **Risk, stated rather than hidden:** `research/mob-behaviour.md` §4.1 shows level > 1 opening
  a block that dereferences `[mob+0x2c0]+0xf84` with no null guard when `template+0x178` is 1
  or 2 - the same shape as the crash `research/mob-spawn.md` §2g solved. Our flow runs
  `0x03C6` first, and `encodeInit` creates `mob+0x2c0` at `141c50eed`, so it should not fire.
  "Should not" is an argument, not a measurement, and a fault here would cost the run.

That risk is exactly why the probe is the recommended variant: it changes nothing on the wire,
so it cannot fault the client, and it answers the bigger question.
