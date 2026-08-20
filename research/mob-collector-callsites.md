# Every call site of `FUN_141d31b20`, and where argument 4 comes from

Written 2026-08-20 for one purpose: **the in-process probe is about to print a return address
at the collector's entry, and this file is the lookup table that turns that number into a
path.** It also traces argument 4 and argument 17 at every call site that can produce a
`0x00DF`, so the run is readable the moment it lands.

Markers as elsewhere: **[L]** read off a listing or capture, **[D]** derived from two or more
[L], **[I]** inferred.

> **SETTLED 2026-08-20, no client run needed.** Measured against
> `research/fixtures/melee-collector-runs-once-per-swing-hook.log`: all six collector entries
> carry `called-from=0x141d2545a` and `r9=0xf`. **Arm C, argument 4 = 15**, exactly as §4.2 and
> §7 predict. The early-out is dead; the loop ran with room for fifteen and accepted zero.
> The follow-on gate analysis for arm C is in **`research/mob-gates-arm-c.md`**, which finds the
> rejecting gate is **geometric** (`141d327c6`) and is not in `mob-target-gates.md` §4's table
> at all.

Companion listings written beside this file:

| file | what |
|---|---|
| `research/msexe-attack-builder-1428c1fa0.txt` | `FUN_1428c1fa0`, `0x1428c1fa0..0x1428c5a93`, 3640 lines |
| `research/msexe-mobtarget-collect-listing.txt` | `FUN_141d31b20`, the collector, 1038 lines |
| `research/msexe-mobtarget-collect-25360.txt` | `FUN_141d25360`, the **third** collector, 138 lines |

---

## 0. Answer up front

| | |
|---|---|
| **the doubt in `mob-target-gates.md` §6.1 is closed** | `FUN_141d31b20` **does** run for a bare melee swing. PROVEN by timing on a real client - six swings, six collector entries, each 1-2 ms before its outbound `0x00DF`, one-to-one, no unpaired events (`research/fixtures/melee-collector-runs-once-per-swing-{world,hook}.log`). §6.1's "it is not established that this function runs for a plain User Melee swing" no longer stands |
| **86 call sites, confirmed by enumeration** | and **all 86 are `call rel32`**. Zero tail `jmp`s, zero `lea rip`, **zero qword pointers anywhere in the image** - so the collector is never entered through a vtable or a function-pointer table. **Every entry the probe sees has a return address in the table below.** §1 |
| **the third collector calls the first one** | `FUN_141d25360` - one of the three collectors in `FUN_1428c1fa0` - is itself **row 29 of the table**: it calls `FUN_141d31b20` at `141d25455`. Nothing in `research/` recorded this. It means the "which of the three collectors runs" question and the "does `FUN_141d31b20` run" question are **not** alternatives. §3 |
| **only three of the 74 containing functions are reachable from a `0x00DF` builder** | `0x1428c1fa0` itself, `0x141d25360` (depth 1), `0x141d275d0` (depth 2). Saturated at depth 4 while the reachable set grew from 648 to 3910 functions. The other two `0x00DF` builders reach **none** at depth 4 - a negative with a positive control in the same run. §2 |
| **so the probe has exactly four possible return addresses** | `0x1428c2c32`, `0x141d2545a`, `0x141d277c9`, `0x141d2783d`. Each carries a different argument 4, and they are 1, 15, 1 and 1. §4 |
| **argument 4 at `1428c2c2d` is `min(mobCount, 15)` and it CAN be zero** | `esi = FUN_1407ea780(skillRecord, skillLevel, ...)`, a 71-byte function that returns **0** when the skill record is null or the level is `<= 0`, otherwise `min(deobfuscated(levelData+0x2a4), 15)`. There is no lower clamp. §4.1 |
| **argument 4 at `141d25455` is the literal 15** | `xor edx,edx` then `lea r9d,[rdx+0xf]`. It cannot be `<= 0`, so on that path the early-out is structurally unable to fire. §4.2 |
| **argument 17 is 0 at all four sites** | verified independently at each, from the caller's own stack stores. §4.4 |
| **and argument 17 is not just a start index - it is the output cursor** | `[rbp+0x620]` is **incremented** inside the loop at `141d327de` and `141d32939`, and `141d31c96 cmp rax,rcx / jge` is re-evaluated at the **top of every iteration** (`141d32a62 jmp 0x141d31c90`). So argument 4 is a **capacity**, not a one-shot precondition, and `mob-target-gates.md` §4.2's "if argument 4 <= argument 17 the loop never starts" is right about the first iteration and understates what the test is. §5 |

---

## 1. The enumeration, and the instrument that produced it

`tools/callers.py` says **86 call sites in 74 functions**. I did not inherit that number.

**Positive control first**, from its own docstring: `python tools/callers.py 0x1402fa9a0` gives
96 sites in 15 functions, with **43 in `0x140304b20`, first `0x140304e49`, last
`0x1403091e7`** - exactly the control the docstring records. [L]

Then an independent enumerator, run from the repo root and piped over stdin so `sys.path[0]`
is empty (`CLAUDE.md` § "The scratchpad shadows the real tools"). It does four scans, not one:

| scan | over | for `0x141d31b20` | control |
|---|---|---|---|
| `call rel32` (`E8`) | every executable section | **86** | 96 for `0x1402fa9a0`, matching `callers.py` |
| `jmp rel32` (`E9`) - a tail call | every executable section | **0** | - |
| `lea reg,[rip+d]` | every executable section | **0** | - |
| the 8-byte pointer value, anywhere | **every** section with raw data | **0** | **1** for `FUN_141cbcad0`, at `0x1434078b8` in `.rdata` |

The pointer scan's control matters most, because it is the one that would hide an indirect
call. `FUN_141cbcad0` is `mob-target-gates.md` §2.1's documented `mobvtbl+0xd0` entry; the same
scan finds its single `.rdata` slot and zero `call rel32`. So **the zero for `0x141d31b20` is a
property of the client, not of the search**: nothing takes its address, nothing tail-jumps to
it, and it appears in no vtable. [L]

Every one of the 86 candidates was then **validated against a linear disassembly of its
containing `.pdata` function** rather than trusted from the byte scan - `reads.py`'s docstring
records that a bare `0xE8` scan lands mid-instruction and silently loses reads. All 86 are real
instruction boundaries, all are `call`, **all are exactly 5 bytes**, and none is prefixed. So
`return address = call address + 5` throughout, with no exceptions. [L]

**Coverage caveat, stated rather than hidden.** `.themida` (0x13ec000 bytes of virtual size)
has a raw size of **0**, so there is nothing on disk to scan there; it is filled at runtime.
`.text` (52 MB) and `.boot` (12 MB) were both covered. A code pointer computed arithmetically -
a table base plus an index, the shape `mob-target-gates.md` §6.1 found for the attack-type
name strings - would also be invisible to all four scans. [I]

---

## 2. Which call sites are on an attack path

Forward call-graph reachability from each of the three `0x00DF` builders, over direct
`call`/`jmp rel32` only, intersected with the 74 containing functions:

```
FUN_1428c1fa0  depth 1:  200 fns,  2 homes: 0x141d25360, 0x1428c1fa0
FUN_1428c1fa0  depth 2:  648 fns,  3 homes: 0x141d25360, 0x141d275d0, 0x1428c1fa0
FUN_1428c1fa0  depth 3: 1792 fns,  3 homes: (same)
FUN_1428c1fa0  depth 4: 3910 fns,  3 homes: (same)

FUN_1428baa50  depth 4: 1704 fns,  0 homes
FUN_1429a6590  depth 4: 1232 fns,  0 homes
```

Two things to read off that. The count **saturates at 3 from depth 2 onward** while the
reachable set grows six-fold, so it is a stable answer and not a depth artefact. And the two
zeros come from the same run that produced the 3, which is the positive control the zeros
need - `mob-target-gates.md` §6.1's finding that `FUN_1428baa50` "touches no mob code at all"
now holds transitively to four levels, not just directly. [L]

The depth-2 path is `FUN_1428c1fa0 --(1428c380a)--> FUN_1428cb4a0 --(1428cba9f)-->
FUN_141d275d0`, and `1428c380a` sits at line 1565 of the builder listing - **after** all three
collectors and after the point where the three arms converge, so it runs on every arm. [L]

---

## 3. The table

Sorted by **return address**, which is the number the probe prints. 86 rows.

| return addr | call at | containing fn | on an attack path? |
|---|---|---|---|
| `0x14074f9ec` | `0x14074f9e7` | `0x14074f7a0` | |
| `0x140750d9d` | `0x140750d98` | `0x140750bd0` | |
| `0x140d5a22d` | `0x140d5a228` | `0x140d59fc0` | |
| `0x140e85445` | `0x140e85440` | `0x140e84770` | |
| `0x1413f04d7` | `0x1413f04d2` | `0x1413f03f0` | |
| `0x14140e0ed` | `0x14140e0e8` | `0x14140d940` | also calls `0x141d25360` |
| `0x14140e2c6` | `0x14140e2c1` | `0x14140d940` | also calls `0x141d25360` |
| `0x14140e8cf` | `0x14140e8ca` | `0x14140e7de` | |
| `0x14140f089` | `0x14140f084` | `0x14140ed10` | |
| `0x14140fdbc` | `0x14140fdb7` | `0x14140f9b0` | |
| `0x14141496a` | `0x141414965` | `0x141414710` | |
| `0x1419d3a5c` | `0x1419d3a57` | `0x1419d3850` | |
| `0x1419d596e` | `0x1419d5969` | `0x1419d5910` | |
| `0x1419d5dbf` | `0x1419d5dba` | `0x1419d5cf0` | |
| `0x1419d7ea8` | `0x1419d7ea3` | `0x1419d7e05` | |
| `0x1419d93e0` | `0x1419d93db` | `0x1419d89c0` | |
| `0x1419e702d` | `0x1419e7028` | `0x1419e6f40` | |
| `0x1419e7099` | `0x1419e7094` | `0x1419e6f40` | |
| `0x141a91dec` | `0x141a91de7` | `0x141a91bb0` | |
| `0x141c6d571` | `0x141c6d56c` | `0x141c6d1d0` | |
| `0x141c6da02` | `0x141c6d9fd` | `0x141c6d690` | |
| `0x141c6eec4` | `0x141c6eebf` | `0x141c6de40` | also calls `0x141d25360` |
| `0x141c6fedb` | `0x141c6fed6` | `0x141c6f830` | |
| `0x141c7235f` | `0x141c7235a` | `0x141c71f60` | |
| `0x141c72a11` | `0x141c72a0c` | `0x141c724c0` | |
| `0x141ca70d2` | `0x141ca70cd` | `0x141ca6fb0` | |
| `0x141cc6741` | `0x141cc673c` | `0x141cc6000` | |
| `0x141cf62a2` | `0x141cf629d` | `0x141cf5f40` | |
| **`0x141d2545a`** | **`0x141d25455`** | **`0x141d25360`** | **YES - the third collector, called by the `0x00DF` builder at `1428c32e9`. arg4 = 15** |
| `0x141d2564f` | `0x141d2564a` | `0x141d25590` | |
| `0x141d25932` | `0x141d2592d` | `0x141d25810` | |
| `0x141d25fb0` | `0x141d25fab` | `0x141d25de0` | |
| `0x141d26474` | `0x141d2646f` | `0x141d262a0` | |
| `0x141d266aa` | `0x141d266a5` | `0x141d26510` | |
| **`0x141d277c9`** | **`0x141d277c4`** | **`0x141d275d0`** | **YES (depth 2) - via `FUN_1428cb4a0` at `1428c380a`. arg4 = 1, arg6 = a single object id** |
| **`0x141d2783d`** | **`0x141d27838`** | **`0x141d275d0`** | **YES (depth 2) - same, arg6 = 0** |
| `0x141d27c52` | `0x141d27c4d` | `0x141d27890` | |
| `0x141d27f44` | `0x141d27f3f` | `0x141d27890` | |
| `0x141d285c4` | `0x141d285bf` | `0x141d28479` | |
| `0x141d28b6a` | `0x141d28b65` | `0x141d28960` | |
| `0x141d28b8d` | `0x141d28b88` | `0x141d28960` | |
| `0x141d28dab` | `0x141d28da6` | `0x141d28bf0` | |
| `0x141d28e83` | `0x141d28e7e` | `0x141d28bf0` | |
| `0x141d29098` | `0x141d29093` | `0x141d28ec0` | |
| `0x141d2ac43` | `0x141d2ac3e` | `0x141d2aa10` | |
| `0x141d2b25f` | `0x141d2b25a` | `0x141d2b170` | |
| `0x141d2bb04` | `0x141d2baff` | `0x141d2ba40` | |
| `0x141d2bd36` | `0x141d2bd31` | `0x141d2bc10` | |
| `0x141d2bfd1` | `0x141d2bfcc` | `0x141d2beb0` | |
| `0x141d2c53f` | `0x141d2c53a` | `0x141d2c4c0` | |
| `0x141d2c7be` | `0x141d2c7b9` | `0x141d2c6c0` | |
| `0x141d2c998` | `0x141d2c993` | `0x141d2c8c0` | |
| `0x141d2cb5d` | `0x141d2cb58` | `0x141d2caa0` | |
| `0x141d2ce0a` | `0x141d2ce05` | `0x141d2ccf0` | |
| `0x142016302` | `0x1420162fd` | `0x142015b50` | |
| `0x1420b169e` | `0x1420b1699` | `0x1420b0ca0` | |
| `0x1420b4ee3` | `0x1420b4ede` | `0x1420b4710` | |
| `0x1420b516a` | `0x1420b5165` | `0x1420b4710` | |
| `0x1420b7fe3` | `0x1420b7fde` | `0x1420b7300` | |
| `0x1420b819e` | `0x1420b8199` | `0x1420b7300` | |
| `0x1420b82bc` | `0x1420b82b7` | `0x1420b7300` | |
| `0x1420ba729` | `0x1420ba724` | `0x1420b9eb0` | |
| `0x1420ba813` | `0x1420ba80e` | `0x1420b9eb0` | |
| `0x1420bebe9` | `0x1420bebe4` | `0x1420be9b0` | |
| `0x1420bfc66` | `0x1420bfc61` | `0x1420bf5c0` | |
| `0x1420c1f50` | `0x1420c1f4b` | `0x1420c1ac0` | |
| `0x1427ac0f4` | `0x1427ac0ef` | `0x1427abc00` | |
| **`0x1428c2c32`** | **`0x1428c2c2d`** | **`0x1428c1fa0`** | **YES - the known `0x00DF` builder, first collector. arg4 = `esi`, see §4.1** |
| `0x1428c8c19` | `0x1428c8c14` | `0x1428c8120` | |
| `0x1428c8ed4` | `0x1428c8ecf` | `0x1428c8120` | |
| `0x1428d31d7` | `0x1428d31d2` | `0x1428d2b70` | |
| `0x14291e57d` | `0x14291e578` | `0x14291e2b0` | |
| `0x142970ce1` | `0x142970cdc` | `0x14296fd50` | |
| `0x142975cd0` | `0x142975ccb` | `0x1429755a0` | |
| `0x1429810b0` | `0x1429810ab` | `0x142980e20` | |
| `0x142983bc7` | `0x142983bc2` | `0x142983290` | |
| `0x142983e79` | `0x142983e74` | `0x142983290` | |
| `0x142984425` | `0x142984420` | `0x142984090` | |
| `0x14298804e` | `0x142988049` | `0x142987880` | |
| `0x14298a5f6` | `0x14298a5f1` | `0x142989f60` | |
| `0x14298c974` | `0x14298c96f` | `0x14298c750` | |
| `0x1429a17bd` | `0x1429a17b8` | `0x1429a15d0` | also calls `0x141d25360` |
| `0x1429a22d3` | `0x1429a22ce` | `0x1429a2110` | |
| `0x1429a2747` | `0x1429a2742` | `0x1429a24a0` | |
| `0x1429a34ba` | `0x1429a34b5` | `0x1429a33b0` | |
| `0x142cec24d` | `0x142cec248` | `0x142cebd90` | |

**None** of the other 82 rows lives in a function that also calls the `0x00DF` builder
`FUN_1428c1fa0` (its 37 caller functions and these 74 have an empty intersection), and none
is reachable from `FUN_1428baa50` or `FUN_1429a6590` at depth 4. The three "also calls
`0x141d25360`" rows are marked because they share a helper with the attack path, not because
they are on it. [L]

---

## 4. Argument 4 and argument 17, traced at every attack-path site

### 4.1 At `1428c2c2d`: `arg4 = esi = FUN_1407ea780(skillRecord, skillLevel, pool, ?, 0)`

`mob-target-gates.md` §4.2 says argument 4 is `esi` and stops there. `esi`'s last write before
`1428c2c19 mov r9d, esi` is **`1428c2ae2 movsxd rsi, eax`**, 0x137 bytes earlier, and there is
no other write to `esi`/`rsi` in between. So `esi` is the sign-extended return of the call at
`1428c2add`: [L]

```asm
1428c2ac5  xor  eax, eax
1428c2ac7  mov  dword ptr [rsp + 0x20], eax   ; 5th argument = 0
1428c2acb  mov  r9,  qword ptr [rbp - 0x30]
1428c2acf  mov  r14, qword ptr [rbp - 0x28]
1428c2ad3  mov  r8,  r14
1428c2ad6  mov  edx, dword ptr [rbp - 0x7c]   ; = FUN_1428c1fa0 argument 4
1428c2ad9  mov  rcx, qword ptr [rbp - 0x60]   ; = FUN_1428c1fa0 argument 3
1428c2add  call 0x1407ea780
1428c2ae2  movsxd rsi, eax                    ; <<< THIS IS ARGUMENT 4
```

`FUN_1407ea780` is **71 bytes** and it is the whole answer: [L]

```asm
1407ea780  sub    rsp, 0x28
1407ea784  test   rcx, rcx
1407ea787  je     0x1407ea7c0        ; -> return 0
1407ea789  test   edx, edx
1407ea78b  jle    0x1407ea7c0        ; -> return 0        <<< edx <= 0 RETURNS ZERO
1407ea78d  call   0x14079fe90        ; rcx, rdx, r8, r9 untouched -> same arguments
1407ea792  mov    edx, dword ptr [rax + 0x2ac]
1407ea798  lea    rcx, [rax + 0x2a4]
1407ea79f  call   0x1401ba9d0        ; deobfuscate the dword at +0x2a4
1407ea7a4  add    eax, dword ptr [rsp + 0x50]   ; + the 5th argument, which is 0 here
1407ea7a8  mov    edx, 0xf
1407ea7ad  cmp    eax, edx
1407ea7af  mov    ecx, edx
1407ea7b1  cmovl  ecx, eax
1407ea7b4  cmp    ecx, edx
1407ea7b6  cmovl  edx, ecx           ; min(eax, 15), written twice - redundant
1407ea7b9  mov    eax, edx
1407ea7bf  ret
1407ea7c0  xor    eax, eax           ; the two early returns
1407ea7c6  ret
```

`[rsp+0x50]` is the 5th argument: after `sub rsp,0x28` the return address is at `[rsp+0x28]`,
the shadow space at `[rsp+0x30..0x48]`, so `[rsp+0x50]` is the first stack argument. The caller
stores `0` there. [D]

**`FUN_1401ba9d0` is the obfuscated-integer getter**, not a general call: [L]

```asm
1401ba9d9  mov  r8d, dword ptr [rcx]
1401ba9dc  mov  ebx, dword ptr [rcx + 4]
1401ba9df  rol  ebx, 5
1401ba9e2  xor  ebx, r8d                  ; ebx = the real value  <- returned
1401ba9e5  xor  r8d, 0xbaadf00d
1401ba9ec  ror  r8d, 5
1401ba9f0  add  r8d, dword ptr [rcx + 4]
1401ba9f9  cmp  r8d, edx                  ; edx = the caller's expected checksum
1401ba9fc  je   0x1401baa54               ; mismatch -> a tamper report, then return anyway
```

So each obfuscated field is 12 bytes - value at `+0`, key at `+4`, checksum at `+8` - and
`FUN_14079fe90(record, level)` returns a per-level data block in which:

| offset | what | evidence |
|---|---|---|
| `+0x2a4` (checksum `+0x2ac`) | **the target-count cap** | it is the only thing `FUN_1407ea780` reads, and `FUN_1407ea780`'s only consumer is argument 4 [D]. Candidate name `mobCount`, from `ModernMapleSource` - **a candidate, not a fact** [I] |
| `+0x5e0`, `+0x5ec`, `+0x5f8`, `+0x604` | the four dwords of the attack rect | read at `1428c2af8..1428c2b39` straight into `[rbp+0x520..0x52c]`, which is the collector's argument 2 [L] |

**So: `argument 4 = min(mobCount, 15)`, or 0 if the skill record is null or the level is
`<= 0`. There is no lower clamp - a `mobCount` of 0 or a negative one passes straight through.**
[D]

### 4.2 At `141d25455`: `arg4` is the literal 15

```asm
141d2539f  xor  edx, edx
...
141d2544a  lea  r9d, [rdx + 0xf]     ; = 15
141d2544e  lea  r8,  [rbp - 0x50]    ; a local 16-slot output array
141d25452  mov  rdx, r10             ; the rect, forwarded from the caller's r8
141d25455  call 0x141d31b20
141d2545a  test eax, eax
141d2545c  jle  0x141d2555a          ; <= 0 -> return 0
```

`edx` is zeroed at `141d2539f` and nothing writes it before `141d2544a`. **Argument 4 is 15,
unconditionally.** On this path the early-out at `141d31c99` is structurally unable to fire on
the first iteration. [L]

`FUN_141d25360` forwards its own arguments 6..17 into the collector's arguments 5..16 and
zeroes 17, 18 and 19. Its own argument 5 (`[rbp+0xa0]`) is a **copy-out** limit used at
`141d254ee`, after the collector has returned - it decides how many of the collected targets
get copied into the caller's array, not how many get collected. [L]

The caller passes that argument 5 as `r14d` at `1428c32cb`, and `r14d` is
`max(deobfuscated(levelData+0x2a4), 1)`: [L]

```asm
1428c318c  mov   r13d, dword ptr [rbp - 0x7c]
1428c3190  test  rsi, rsi                  ; rsi = the skill record
1428c3193  je    0x1428c31bf               ; NULL is tolerated here
1428c319b  call  0x14079fe90
1428c31a0  lea   rcx, [rax + 0x2a4]        ; the SAME field as FUN_1407ea780 reads
1428c31ad  call  0x1401ba9d0
1428c31b2  mov   r14d, 1
1428c31b8  cmp   eax, r14d
1428c31bb  cmovg r14d, eax                 ; max(mobCount, 1)   <<< FLOORED AT 1
```

**The same WZ field is read on both paths and clamped differently: `min(x,15)` with no floor
on the `1428c2c2d` path, `max(x,1)` on the `1428c32e9` path.** That asymmetry is the whole
difference between "collects nothing" and "collects at least one". [D]

### 4.3 At `141d277c4` and `141d27838`: `arg4 = 1`

`141d2775b mov r9d, 1`, set before the `je 0x141d277e6` that splits the two calls, and not
rewritten on either arm. The first call also passes argument 6 = `[rsp+0xa0]` (a single object
id, which arms gate 4); the second passes 0. [L]

### 4.4 Argument 17 is 0 at all four sites - verified independently

The caller-side slot for argument *N* is `[rsp + 0x20 + 8(N-5)]`, so argument 17 is
`[rsp+0x80]`.

| site | the store | value |
|---|---|---|
| `1428c2c2d` | `1428c2bd9 mov dword ptr [rsp+0x80], ebx`, `ebx` zeroed at `1428c2bc8 xor ebx,ebx` | **0** [L] |
| `141d25455` | `141d253d4 mov dword ptr [rsp+0x80], r14d`, `r14d` zeroed at `141d253c1` | **0** [L] |
| `141d277c4` | `141d27783 mov dword ptr [rsp+0x80], ecx`, `ecx` zeroed at `141d27772` | **0** [L] |
| `141d27838` | `141d277ff mov dword ptr [rsp+0x80], eax`, `eax` zeroed at `141d277ee` | **0** [L] |

**And the argument map itself was re-derived rather than inherited.** The collector's prologue
is `mov [rsp+8],rbx` then 7 pushes then `lea rbp,[rsp-0x560]`, which puts argument *N* at
`rbp + 0x5c0 + 8(N-5)`. Two independent checks that it is right, both from the collector's own
code: argument 5 is read at `141d31b5e` as `[rbp+0x5c0]`, and **argument 14 is read at
`141d31b81` as `[rbp+0x608]` and handed straight to `FUN_1407b2910`, the skill-node lookup** -
which is exactly what gate 6 and gate 17 need argument 14 to be. Argument 17 is therefore
`[rbp+0x620]`, as `mob-target-gates.md` §4.1 says. [D]

A third, independent cross-check falls out of §4.2: `FUN_1428c1fa0` puts the skill id in
**its** argument 15 to `FUN_141d25360` (`1428c32a1 mov [rsp+0x70], r15d`), and
`FUN_141d25360` forwards its argument 15 into the collector's **argument 14**
(`141d253eb/253f1`). Two different call chains, two different stack layouts, the skill id
lands in argument 14 both times. [D]

There is also an **argument 19** slot written at both `1428c2bca` and `141d253c4`
(`[rsp+0x90]`). `mob-target-gates.md` §4.1's map stops at 18. Nothing in the collector was
found reading `[rbp+0x630]`; not chased. [L]

---

## 5. What the early-out actually is

`mob-target-gates.md` §4.2 reads `141d31c96 cmp rax,rcx / jge` as a one-shot precondition. The
listing says it is the **loop header**:

```asm
141d31c64  mov    rbx, qword ptr [r14 + 0x58]   ; the mob list head
141d31c70  je     0x141d32a67                   ; empty list -> return
141d31c76  movsxd rax, dword ptr [rbp + 0x620]  ; rax = argument 17
141d31c7d  mov    qword ptr [rsp + 0x58], rax
141d31c82  mov    rcx, r15                      ; r15 = movsxd(argument 4), set at 141d31b50
>141d31c90 mov    esi, dword ptr [rbp + 0x5f8]  ; <<< LOOP TOP
141d31c96  cmp    rax, rcx
141d31c99  jge    0x141d32a67                   ; RETURN
...
141d327de  inc    dword ptr [rbp + 0x620]       ; the cursor advances on an accept
141d32939  inc    dword ptr [rbp + 0x620]
...
141d32a59  mov    rax, qword ptr [rsp + 0x58]
141d32a5e  mov    rcx, qword ptr [rbp - 0x40]
141d32a62  jmp    0x141d31c90                   ; <<< BACK TO LOOP TOP
```

So `[rbp+0x620]` is the **output cursor**, argument 17 is where it starts, and argument 4 is
the **capacity**. `141d31c70` is a second, earlier return: **an empty mob list returns before
the cursor is even loaded.** The return value is `([rsp+0x38] << 16) | [rbp+0x620]` - the
accept count in the high half and the *final* cursor in the low half, not the initial argument
17. [L]

That last point sharpens §4.2's second consequence: `1428c2c32 mov [rbp+0x870], eax` stores a
packed pair whose low half is a **count of targets written**, and the caller's very next test
`1428c2c44 cmp dword ptr [rbp+0x870], esi / jge` compares that packed dword against the
capacity. With a non-zero accept count the high half alone makes the value >= 0x10000, so the
`jge` is always taken and the follow-up collector at `1428c2c65` is skipped; with zero accepts
it falls through. **That branch is a "did I get nothing?" test, whether or not it was written
as one.** [D]

---

## 6. Deliverable 3: which of the three collectors runs for a bare swing

`FUN_1428c1fa0` has three collectors writing `[rbp+0x870]`, and **two more calls to
`FUN_141d35440` that also write it** (`1428c2c65`, `1428c3327`) which no existing note records.
They are **not** sequential and they are **not** conditional on each other's results. They are
three mutually exclusive arms of one dispatch: [L]

```
1428c2911  mov  r13, [rip+0x11fd4e8]      ; -> *(0x143ABFE00), the mob pool singleton
1428c291b  jne  0x1428c29b6               ; null -> the function bails out entirely
1428c29b6  mov  r15, [rip+0x11fd7cb]      ; -> *(0x143AC0188), a second singleton
1428c29c0  jne  0x1428c2a63               ; null -> bails out
   |
   +-- 1428c2a63  mov rsi, [rbp-0x60]     ; the skill record = FUN_1428c1fa0 argument 3
       1428c2a6a  je  0x1428c2cbf         ; NULL -> arm B/C
       1428c2a9d  jge 0x1428c2cbb         ; rect left >= right  -> arm B/C
       1428c2abf  jge 0x1428c2cbb         ; rect top  >= bottom -> arm B/C
       ...
       1428c2c2d  call 0x141d31b20        ; ARM A
       1428c2caa  jmp  0x1428c3564        ; and OUT - never reaches the other two
   |
   +-- 1428c2cbf  mov ecx, [rbp-0x78]     ; the skill id
       1428c2cc2  call 0x1407e6fe0        ; returns 1 only for id 0x17D84B81 or 0x17D84B89
       1428c2cc9  je  0x1428c318c         ; FALSE (and 0 is false) -> arm C
       ...
       1428c30b1  call 0x141d2a4f0        ; ARM B
       1428c3187  jmp  0x1428c3564        ; and OUT
   |
   +-- 1428c318c  ...                     ; ARM C, tolerates a NULL skill record
       1428c32e9  call 0x141d25360        ; -> which calls 0x141d31b20 at 141d25455
```

`FUN_1407e6fe0` and `FUN_1407ecff0` were read rather than guessed: `FUN_1407ecff0` is
`xor eax,eax / test ecx,ecx / sete al / ret`, i.e. **`skillId == 0`**, and it is what gates the
extra `FUN_141d35440` call on both arm A and arm C. `FUN_1407e6fe0` is
`sub ecx,0x17D84B81 / je / cmp ecx,8 / je`, i.e. two specific ids and nothing else. [L]

Every collector call in the builder takes its pool from the **same singleton**,
`*(0x143ABFE00)` - checked by resolving all five rip-relative operands
(`1428c2911`, `1428c2c5e`, `1428c30aa`, `1428c32e2`, `1428c3320`). So the three arms differ in
which *walker* they use, not in which pool they walk. [L]

### Which arm is a bare "User Melee" swing?

Not settled by static analysis, and I am not going to write it up as if it were. What the
listing does say: [L]

* Arm A **requires a non-null skill record** and a non-degenerate skill rect, and its rect
  comes out of `FUN_14079fe90(skillRecord, level) + 0x5e0` - i.e. the skill's own WZ rect.
* Arm C **explicitly tolerates a null skill record** (`1428c3193 je 0x1428c31bf`), and its
  rect is built by `FUN_140ce1950(*(0x143AC0188), &user, ..., &[rbp+0x520], <a double>)`
  rather than read out of a skill record.
* Both arms carry an explicit `skillId == 0` sub-branch.

**[I]** A null-check that a bare swing could never reach would be dead code, so arm C reading
like the no-skill arm is suggestive - but that is an inference about a branch, exactly the
shape this project has been burned by, and the probe settles it in one run.

**And it does not matter for the instrument**, because both arms end in `FUN_141d31b20`. The
timing fact - one collector entry per swing, 1-2 ms before the `0x00DF` - is consistent with
arm A (one call at `1428c2c2d`) and with arm C (one call at `141d25455`) alike. The **return
address is what separates them**, and that is what the probe prints.

---

## 7. The reading, written before the run

**Predicted values.**

| if the return address is | then argument 4 is | argument 17 | prediction |
|---|---:|---:|---|
| `0x1428c2c32` | `min(mobCount, 15)`, **0 if the skill record is null or the level `<= 0`** | 0 | **N = 1** [I] - a bare weapon swing hits one mob, and a `mobCount` of 1 for a basic attack is the ordinary case. **N = 0 is the interesting outcome and it is a live possibility**, because this path has no floor |
| `0x141d2545a` | **15**, hardcoded | 0 | **N = 15**, and there is nothing that can change it [L] |
| `0x141d277c9` / `0x141d2783d` | **1**, hardcoded | 0 | N = 1 [L] |
| anything else in §3 | - | - | the swing is not going through a `0x00DF` builder and §4 of `mob-target-gates.md` is the wrong instrument |

**The two readings the task asked for, explicitly:**

> **"argument 4 = N, argument 17 = 0, N > 0"** - the loop **did** run. The early-out did not
> fire, at least one mob was examined, and the rejection is per-mob after all. That sends the
> next pass to the places `mob-target-gates.md` did **not** read: **gates 18-24**, the block
> behind `FUN_14047c1d0(template)` returning a non-empty list at `141d321df` (§7 item 3 of that
> file, seven reject branches, none read), and to gate 4 (`arg6 = [rbp+0x6f80]`, a single
> object id) if arg6 comes back non-zero. It also makes `141d31c70` worth a second look -
> `[pool+0x58]` being null returns *before* the cursor is loaded, and §5 of `mob-target-gates.md`
> derived that `pool+0x58` is the right list from the *insert* side only.
>
> **"argument 4 <= argument 17"** - the early-out fired, no mob was ever examined, and every
> gate in §4 of `mob-target-gates.md` is irrelevant. Since argument 17 is 0 at all four sites,
> this means **argument 4 <= 0**, and on the `0x1428c2c32` path that has exactly three causes,
> in order of likelihood: the deobfuscated `mobCount` at `levelData+0x2a4` is 0; the skill
> level (`FUN_1428c1fa0` argument 4) is `<= 0`; the skill record is null. **On the
> `0x141d2545a` path this outcome is impossible**, so seeing it there would mean the argument
> map is wrong and nothing above should be trusted.

**Is argument 4 a client-side maximum, and could a server influence it?**
Yes to the first, and **no to the second, on the evidence here.** It is a client-side cap
derived from an obfuscated integer inside the WZ-loaded per-level data block
(`FUN_14079fe90(record, level) + 0x2a4`), clamped to 15, with the 15 itself matching the
15-element target array `142ef44fc(&[rbp+0x5c0], 0x1d8, 0xf, ...)` that `FUN_1428baa50`
serialises. The only input a server touches at all is the **skill level**, and only if the
attack is a skill - which a bare swing is not. **No packet we send reaches `+0x2a4`.** [D]

---

## 8. Ranked watch addresses for the next run

Ranked for one launch, on the assumption the probe prints the return address, `r8`, `r9` and
the stack arguments from `[rsp+0x28]` up at a function entry.

| rank | watch | why | what each outcome means |
|---:|---|---|---|
| **1** | **`0x141d31b20`** (the collector itself) | it is already known to fire once per swing, and §1 proves **every** entry to it carries one of the 86 return addresses in the table. `r9d` **is** argument 4, so one watch answers both halves of the question at once | ret `0x141d2545a` -> arm C, `r9d` must read 15; ret `0x1428c2c32` -> arm A, `r9d` is the number that decides everything; ret `0x141d277c9`/`0x141d2783d` -> the post-collector single-target query, and the real collector fired elsewhere; any other ret -> not an attack path |
| **2** | **`0x1428c1fa0`** (the `0x00DF` builder entry) | closes `mob-target-gates.md` §7 open question 1 outright, and its return address names which of the 37 caller functions is the melee path - the thing three whole-image string sweeps failed to find. Its `r8` is the **skill record** and `r9d` the **skill level**, which are precisely the two inputs that can drive argument 4 to 0 | `r8 == 0` -> arm C is forced and argument 4 is the hardcoded 15; `r8 != 0, r9d <= 0` -> `FUN_1407ea780` returns 0 and arm A's early-out fires; `r8 != 0, r9d > 0` -> argument 4 is `min(mobCount,15)` and the run needs watch 1 as well |
| **3** | **`0x1407ea780`** (the 71-byte argument-4 producer) | 71 bytes, and its return value **is** argument 4 at `1428c2c2d`. Its `rcx`/`edx` at entry are the skill record and level. Cheapest possible confirmation of the whole §4.1 chain - but only fires on arm A | no entry at all -> arm A did not run, so arm C is the melee path and argument 4 is 15 |
| 4 | `0x141d25360` (the third collector) | 11 call sites; the return address separates `1428c32e9` from the other ten. Redundant with watch 1 if watch 1 is armed, since arm C's collector entry already carries `0x141d2545a` | - |
| 5 | `0x141d35440` | the two calls at `1428c2c65` / `1428c3327` are gated on `skillId == 0` and on the first collector having returned fewer than the cap - so **an entry here is direct evidence that the primary collector came back empty on a bare swing** | entered -> the primary collector returned 0 accepts |

**Watches 1 and 2 together are one run and answer everything above.** Watch 1 alone leaves the
`r9d == 0` case ambiguous between "null skill record" and "level <= 0"; watch 2 alone does not
say whether the loop ran. They do not interact - both are entry watches on functions that are
already going to execute, and neither changes a byte on the wire, so the §9 caution in
`mob-target-gates.md` about perturbing the run still holds in the same weak form it had there.

**One caution carried forward.** `FUN_141d31b20` has 86 call sites in 74 functions and some are
plausibly per-frame, so the hook log may fill before the owner ever swings. `research/mob-spawn.md`
§2g is explicit that the probe's `stack:` line is a heuristic scan and only `called-from=` can
be trusted. Read the return address against §3 and ignore the stack.

---

## 9. Instrument notes

* **`tools/callers.py`'s control reproduces**: 96 sites in 15 functions for `0x1402fa9a0`,
  with 43 in `0x140304b20`, first `0x140304e49`, last `0x1403091e7`. The independent
  enumerator agrees on all three numbers, which is why its 86/74 for `0x141d31b20` is quoted
  rather than `callers.py`'s.
* **`tools/callers.py` was then rewritten by another agent mid-session** to add tail-`jmp` and
  data-pointer scanning - the same two modes the independent enumerator above already had.
  Per `CLAUDE.md`, the table was re-run against the new version rather than assumed safe:
  **86 call sites, 0 tail jmps, 0 qword pointers, no unvalidated hits.** The table stands, and
  the two tools now agree on all three modes.
* **`tools/listing.py`'s control reproduces**: `0x140304100` prints reads at `140304138 raw`,
  `140304144 u8`, `140304183 u8`, then a run of `u16`, exactly as its docstring requires. Every
  listing quoted here came out of it afterwards.
* **The pointer scan has a positive control and it passed** - `FUN_141cbcad0`, the documented
  `mobvtbl+0xd0` entry, gives exactly one `.rdata` hit at `0x1434078b8` and zero `call rel32`.
  Without that, "zero qword pointers to the collector" would have been the fourth clean,
  confident, unverified zero this project has recorded.
* **Every call candidate was validated against a linear disassembly**, not accepted from the
  byte scan. All 86 landed on real instruction boundaries; none was a false positive. Had one
  not, the return address for that row would have been wrong by an arbitrary amount, and the
  whole point of the table is that the number is exact.
* **`FUN_1428c1fa0` is a single `.pdata` extent** (`0x1428c1fa0..0x1428c5a93`, 15091 bytes, no
  merge), so `docs/ghidra.md`'s "bound every dump by `.pdata`" trap does not apply to the
  listing beside this file.
* **No Ghidra was used.** Everything here is capstone over `client-patched\MapleStory.exe`
  through the repo's own loaders, run with the repo as the working directory.
