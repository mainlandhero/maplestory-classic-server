# The gate chain re-run for arm C - and the gate that was never in the table

Written 2026-08-20, after the measurement confirmed the client reaches `FUN_141d31b20` through
**arm C** (`FUN_141d25360` at `141d25455`), not through arm A (`1428c2c2d`).

Markers: **[L]** read off a listing or capture, **[D]** derived from two or more [L], **[I]**
inferred.

Companion listings written beside this file:

| file | what |
|---|---|
| `research/msexe-mobtarget-collect-listing.txt` | `FUN_141d31b20`, the collector |
| `research/msexe-attack-builder-1428c1fa0.txt` | `FUN_1428c1fa0`, the `0x00DF` builder |
| `research/msexe-mobtarget-collect-25360.txt` | `FUN_141d25360`, arm C's collector |
| `research/msexe-mob-hitrects-141ce9a30.txt` | `FUN_141ce9a30`, the `mob+0xe78` rect walker |
| `research/msexe-mobcmd-dispatch-141d32b30.txt` | the `0x03D9..0x044D` per-mob command dispatcher |

---

## 0. Answer up front

| | |
|---|---|
| **the arguments are confirmed to the byte** | `rdx` and `r8` are **stack pointers**, not small integers and not image-relative. Predicted `arg2 - arg3 = 0x750` purely from the two prologues; **measured `0x146530 - 0x145de0 = 0x750`**, and the implied frame then reproduces `arg3` exactly. §1 |
| **but the premise of this task is wrong, and I am saying so rather than confirming it** | **arm C does not switch those six gates on. It switches a seventh one OFF.** Arm C's argument set is *strictly more zeroed* than arm A's: identical everywhere except **arg6**, which arm A may pass non-zero and arm C always passes as 0. Gates 3, 5, 11, 14, 15 and 16 are off under both arms; gate 4 is off under arm C and was the only argument-driven gate that was live under arm A. §2 |
| **the gate that rejects the snail was never in the table** | `mob-target-gates.md` §4 enumerates 17 gates and **not one of them is geometric**. The collector's actual accept for a plain swing is at **`141d327cc`**, guarded by `141d327c6 test r14b,r14b / je` - "did any of this mob's rectangles intersect any of the attack rectangles". §3 classified that branch as *"inside a nested container-growth block, plus two list-walk exits"*. It is the hit test. §3, §4 |
| **and it fails silently on an all-zero rectangle** | the per-rect filter is `141d326ae cmp [rdi],eax / jge` (left >= right) and `141d326ba` (top >= bottom). A **zero** rect satisfies both, so it is **skipped, not rejected** - `r14b` stays 0, the loop finishes, and the mob is dropped with all 17 gates passed. §4.2 |
| **the mob's own rectangle comes from `mob+0x42c`, and the constructor leaves it zero** | `[mobvtbl+0x10] = FUN_141c56e00` -> `[mobvtbl+0x68]` (the stance) -> `FUN_141c57120`, which **bails at `141c573d7` and writes an all-zero 16-byte rect** when `mob+0x42c` is degenerate or `mob+0x98c != 0`. `mob+0x42c` is zeroed by the mob constructor at `141c4d427`, and **every direct write to it anywhere in the image is a zeroing write**. §5 |
| **and the real setter is gated on `mob+0xa88`, which the constructor also leaves null** | the rect is written by `FUN_141cb4600` through a `lea`'d pointer at `141cb4647` - invisible to a `[reg+disp]` write-scan, which is why an earlier draft of §5.3 stated the opposite and is now **retracted in place**. It returns writing nothing when `mob+0xa88` is null (`141cb4645`), and `FUN_141c57120` independently bails on the same field (`141c57185`). **Two gates, one field.** §5.3, §5.5 |
| **so `move_action` is back, by a different door** | `mob-target-gates.md` §2 proves `move_action` "round-trips into the animation state and **nothing else**" and concludes it is **exonerated**. That is right about gates 2 and 8 and wrong as a general clearance: the animation state is the input to `[mobvtbl+0x68]`, which is the input to the rect, which is the input to the only geometric gate. §5.4 |
| **a second, independent candidate that arm A would have caught** | arm A checks its attack rect for `left < right` and `top < bottom` at `1428c2a9d` / `1428c2abf` **before** calling the collector. **Arm C performs no such check.** A degenerate *attack* rect rejects every mob at `141d32713`/`141d32715` with every mob rect perfectly healthy. §6 |
| **what is now known not to be the problem** | argument 4 is 15 and the loop ran (measured). The early-out is dead. Gates 3, 4, 5, 6, 11, 12, 14, 15, 16, 17 are all argument-off under arm C. §2, §3 |

---

## 1. arg1, arg2, arg3 resolved - and the 0x750 is not a coincidence

Measured, all six entries identical:
`rcx=0x37d74fa0  rdx=0x146530  r8=0x145de0  r9=0xf`. [L]

From the two prologues, with no measured input at all: [L]

```
FUN_1428c1fa0:  8 pushes (0x40), lea rbp,[rsp-0x6f08], sub rsp,0x7068
                -> rbp = rsp_final + 0x160
                -> collector arg2 = &[rbp+0x520]  (1428c32d7)  = rsp_final + 0x680
call 0x141d25360 (8) ; FUN_141d25360: 7 pushes (0x38), lea rbp,[rsp-0x40], sub rsp,0x160
                -> rbp_c = rsp_final - 0x80
                -> collector arg3 = &[rbp_c-0x50] (141d2544e)  = rsp_final - 0xd0
```

**Predicted `arg2 - arg3 = 0x750`. Measured `0x146530 - 0x145de0 = 0x750`.** [D]

That fixes `FUN_1428c1fa0`'s frame at `rsp_final = 0x145eb0`, and re-deriving `arg3` from it gives
`0x145eb0 - 0xd0 = 0x145de0` - the measured value, to the byte. Two independent quantities from
one assumption; both land. [D]

| register | is | where it lives |
|---|---|---|
| `rcx = 0x37d74fa0` | **arg1, the mob pool** - `*(0x143ABFE00)`, passed unchanged through `FUN_141d25360` (nothing writes `rcx` between `141d25360` and `141d25455`) | heap |
| `rdx = 0x146530` | **arg2, the attack rect** - `&[rbp+0x520]` in `FUN_1428c1fa0`, four dwords l/t/r/b | main-thread stack |
| `r8  = 0x145de0` | **arg3, the output array** - `&[rbp-0x50]` in `FUN_141d25360`, a 16-slot local | main-thread stack, one frame deeper |
| `r9  = 0xf` | **arg4 = 15**, `xor edx,edx / lea r9d,[rdx+0xf]` | - |

**On the image-relative reading, which the task asked me to consider and which I am rejecting.**
`0x140000000 + 0x146530 = 0x140146530` lands inside `.text` (`0x140001000..0x143261000`), and
`0x140145de0` likewise. Two code addresses used as a rect pointer and an output array would be
nonsense, and it would additionally require the exact `0x750` separation predicted by the frame
arithmetic to be an accident. Both values are ordinary Windows x64 **main-thread stack**
addresses; that thread's stack commonly sits in the low `0x1xxxxx` range. The frame arithmetic
is independent of that convention and settles it on its own. [D]

**So arg2 is a real pointer to a real rect, and "a degenerate rect that is not even a pointer"
is off the table.** What is *in* that rect is a separate question and is still open - see §6.

---

## 2. Arm C's argument set, against arm A's - the premise corrected

`FUN_141d25360` forwards **its own arguments 6..17 into the collector's arguments 5..16** and
zeroes 17, 18 and 19. Verified store by store (`141d2538e`, `141d253dc`..`141d25445`), and its
frame is `rbp_c = R - 0x78`, so `[rbp+0xa8]` is its arg6, `[rbp+0x100]` its arg17. [L]

The caller's stores at `1428c3288..1428c32cb` then give the whole chain: [L]

| collector arg | **arm A** (`1428c2c2d`) | **arm C** (`141d25455`) | gate it drives |
|---:|---|---|---|
| 1 | pool | pool | - |
| 2 | skill rect, **checked** l<r,t<b first | `FUN_140ce1950` rect, **unchecked** | the geometric test |
| 3 | `&[rbp+0x6d20]` | `&[rbp-0x50]` (copied out afterwards) | - |
| 4 | `min(mobCount,15)`, can be 0 | **15** | the loop bound |
| 5 | 0 | 0 | gate 3 - off both |
| **6** | **`[rbp+0x6f80]`, may be non-zero** | **0** | **gate 4 - live under A, OFF under C** |
| 7 | 0 | 0 | - |
| 8 | 0 | 0 | gate 5 - off both |
| 9 | 0 | 0 | gate 9 falls back to template |
| 10 | 0 | 0 | gate 7 |
| 11 | 0 | 0 | gate 11 - off both |
| 12 | 0 | 0 | gate 11 - off both |
| 13 | 0 | 0 | gate 11 |
| 14 | skill id | skill id | gates 6, 12, 17 |
| 15 | 0 | 0 | gate 14 - off both |
| 16 | 0 | 0 | gates 15, 16 - off both |
| 17 | 0 | 0 | the output cursor |
| 18 | 0 | 0 | the extra-rect range at `141d31bdf` |
| 19 | 0 | 0 | unread |

> **The task's premise was that arm A's arguments switched six gates off and arm C's might
> switch them on. The opposite is true.** The two argument sets are identical except for
> argument 6, and arm C is the one that zeroes it. Every gate `mob-target-gates.md` §6
> dismissed as argument-off is **still** argument-off under arm C, and gate 4 - the single
> argument-driven gate that was live under arm A - joins them. Re-running §4 against arm C
> therefore *removes* a candidate rather than adding six.
>
> §6's underlying conclusion ("no per-mob gate in the table rejects a snail") survives the
> correction. What does not survive is the *table*: it is missing a gate. §3.

Two cross-checks that the map is right, both internal to the binary: the skill id lands in the
collector's **argument 14** through both chains (arm A: `1428c2be9 mov [rsp+0x68],r15d`; arm C:
`1428c32a1 -> 141d253eb -> [rsp+0x68]`), and argument 14 is what `141d31b81` feeds to
`FUN_1407b2910`, the skill-node lookup gates 6 and 17 need. [D]

---

## 3. The 27 rejections, re-enumerated and classified under arm C

Re-derived rather than inherited: every branch in the merged extent, bucketed by target, with
no assumption about which labels are the reject labels. **28** branches reach the three labels;
`141d32a06` sits *after* the accept and is not a rejection, exactly as `mob-target-gates.md` §3
records - so **27**, reproducing that count independently. [L]

| # | at | label | what it tests | arm C | can a snail fail it? |
|---:|---|---|---|---|---|
| 1 | `141d31cbf` | a24 | `mob+0x2d8` flag | **live** | no - `0x03C6` sets it |
| 2 | `141d31ce5` | a24 | `mob+0x504` | **live** | no - ctor 0 |
| 3 | `141d31d21` | a24 | arg5 non-null | **off** (arg5=0) | - |
| 4 | `141d31d53` | a24 | arg6 non-zero | **off** (arg6=0) | **- newly off under arm C** |
| 5 | `141d31d84` | a24 | arg8 non-zero | **off** | - |
| 6 | `141d31dbd` | a24 | skill node from arg14 | **off** (id 0 -> null node) | - |
| 7 | `141d31dec` | a24 | `template+0x130` | **live** | no - WZ |
| 8 | `141d31e12` | a24 | `mob+0x300` | **live** | no - ctor -1 |
| 9 | `141d31e40` | a24 | `allyMob` etc. | **live** | no - WZ |
| 10 | `141d31e68` | a24 | `notDamaged` etc. | **live** | no - WZ |
| 11 | `141d31ed0` | a24 | arg11/arg12 distance | **off** (both 0) | - |
| 12 | `141d320ca` | a24 | `sil`, from the skill node | **off** | - |
| 13 | `141d320f0` | a24 | `onlyHittedByCommonAttack` | **live** | no - WZ |
| 14 | `141d3211e` | a24 | arg15 non-zero | **off** | - |
| 15 | `141d32154` | a1d | arg16 non-null | **off** (arg16=0) | - |
| 16 | `141d32177` | a1d | `[arg16+0x70]` vcall | **off** | - |
| 17 | `141d321ae` | a1d | arg14 == 2301001 | **off** (id 0) | - |
| 18-24 | `141d322ab`, `141d322b8`, `141d322f5`, `141d32340`, `141d32392`, `141d323e7`, `141d32409` | a1d | the block entered at `141d321df` when `FUN_14047c1d0(template)` returns a **non-empty** list | **live if the template has that list** | **unknown - still unread** |
| 25 | `141d32644` | a18 | a `jmp` with no branch into it; the fall-through guard of the vector-grow path | dead | no |
| 26 | `141d32692` | a18 | **the mob contributed no rectangles at all** | **live but near-unreachable** - the `[mobvtbl+0x10]` push at `141d32630`/`141d32649` is unconditional, so the vector is non-empty whatever the rect contains | low |
| **27** | **`141d327c6`** | **a18** | **`r14b == 0` - no rectangle of this mob intersected any attack rectangle** | **LIVE** | **YES - this is the candidate** |

**Rows 25-27 are the three that `mob-target-gates.md` §3 summarised as "inside a nested
container-growth block, plus two list-walk exits".** Row 27 is the hit test and row 26 is its
degenerate case. Neither appears anywhere in §4's gate table. [L]

---

## 4. The geometric gate, in full

### 4.1 Where the accept really is

`mob-target-gates.md` §3 locates the accept at the `cmove` counter at `141d329f4` and derives
the rejections as "anything that jumps past it". That is a sound rule, but it hides that there
are **two** places a mob is written into the output array, and the first one is the one a plain
swing uses: [L]

```asm
>141d327cc  mov  rax, qword ptr [rbx + 8]        ; the mob
 141d327d0  mov  rcx, qword ptr [rsp + 0x58]     ; the output cursor
 141d327d5  mov  rdx, qword ptr [rsp + 0x70]     ; argument 3, the output array
 141d327da  mov  qword ptr [rdx + rcx*8], rax    ; <<< THE MOB IS ACCEPTED HERE
 141d327de  inc  dword ptr [rbp + 0x620]         ; the cursor advances
```

and the only thing standing in front of it is:

```asm
 141d327c3  test r14b, r14b
 141d327c6  je   0x141d32a18                     ; <<< no rectangle hit -> REJECT
```

### 4.2 The rect loop, and why a zero rect is silent

```asm
 141d32576  xor   r14b, r14b                       ; the HIT flag starts 0
 141d325bb  mov   rax, [rip -> 0x143AD2C68]        ; vector begin
 141d325c2  mov   [rip -> 0x143AD2C70], rax        ; end := begin -> the vector is CLEARED per mob
 141d325ef  call  0x141c59a30(mob, &vec, 1)        ; filler 1 - template/summon rects, heavily conditional
 141d3260d  call  [mobvtbl+0x10](mob, &[rbp+0xa0], 1) ; filler 2 - the mob's OWN body rect
 141d32630  movups [rdx], xmm0 / add end,0x10      ; ...pushed UNCONDITIONALLY
 141d3267c  call  0x141ce9a30(mob, &vec)           ; filler 3 - the mob+0xe78 array
 141d3268f  cmp   rdi, rsi
 141d32692  je    0x141d32a18                      ; reject 26: vector empty

>141d326a0  test  rdi, rdi
 141d326a3  je    0x141d327b6                      ; null entry -> SKIP
 141d326a9  mov   eax, [rdi + 8]
 141d326ac  cmp   [rdi], eax
 141d326ae  jge   0x141d327b6                      ; left  >= right  -> SKIP THIS RECT
 141d326b4  mov   eax, [rdi + 0xc]
 141d326b7  cmp   [rdi + 4], eax
 141d326ba  jge   0x141d327b6                      ; top   >= bottom -> SKIP THIS RECT
 141d326c0  mov   r8, [rsp + 0x40]                 ; the ATTACK rect vector, begin
>141d326c5  cmp   r8, r12                          ; ...end
 141d326c8  je    0x141d32745
            r9d  = min(mob.bottom, atk.bottom)
            edx  = min(mob.right,  atk.right)
            r10d = max(mob.top,    atk.top)
            ecx  = max(mob.left,   atk.left)
 141d32713  cmp   ecx, edx
 141d32715  jge   0x141d3271c                      ; empty intersection -> next attack rect
 141d32717  cmp   r10d, r9d
 141d3271a  jl    0x141d3272d
>141d3272d  mov   r14b, 1                          ; <<< HIT
>141d327b6  add   rdi, 0x10
 141d327ba  cmp   rdi, rsi
 141d327bd  jne   0x141d326a0                      ; next mob rect
```

**The crucial asymmetry: a degenerate mob rect is `SKIP`ped, not rejected.** An all-zero rect has
`left >= right` (`0 >= 0`) and takes `141d326ae`. If every rect in the vector is skipped, the
loop simply ends with `r14b == 0` and `141d327c6` drops the mob - with no branch anywhere in
`mob-target-gates.md` §4's table having been taken. That is precisely the reported symptom:
**15 slots of room, the mob examined, zero accepted, every documented gate green.** [D]

The attack rect vector at `[rsp+0x40]` is a `vector<RECT>` seeded with **argument 2** by
`141d31bda call 0x1407663f0(&vec, 0, arg2)` - a 16-byte insert (`FUN_1407663f0` is the
vector `_Emplace_reallocate` shape, `r8` = the source element). Argument 18 would append more;
it is 0, so the vector holds exactly one rect: the attack rect. [L]

---

## 5. Where the mob's own rectangle comes from, and why it is probably zero

### 5.1 The vtable slot, derived from a documented anchor

`mob-target-gates.md` §2.1 records `[mobvtbl+0xd0] = FUN_141cbcad0`. The whole-image pointer
scan in `research/mob-collector-callsites.md` §1 found `FUN_141cbcad0`'s single `.rdata` slot at
`0x1434078b8`, so **`mobvtbl = 0x1434077e8`**, and reading it back gives `vtbl+0xd0 ->
0x141cbcad0` - the anchor reproduces. [D]

| slot | value |
|---|---|
| `vtbl+0x00` | `0x141c626c0` |
| **`vtbl+0x10`** | **`0x141c56e00`** - filler 2 |
| `vtbl+0xd0` | `0x141cbcad0` (the documented anchor) |

*Caveat, stated because `docs/ghidra.md` insists on it:* `/OPT:ICF` folds identical functions, so
a vtable slot value is not by itself proof of class identity. This reading is anchored on a
known slot in the same table and on the fact that `FUN_141c56e00` has **zero `call rel32`
sites** - it is reached only indirectly, which is what a vtable-only method looks like. [D]

### 5.2 `FUN_141c56e00` -> the stance -> `FUN_141c57120` -> `mob+0x42c`

```asm
FUN_141c56e00(mob, outRect, flag):
 141c56e0f  mov  rax, [rcx]
 141c56e1b  call qword ptr [rax + 0x68]      ; eax = the mob's CURRENT STANCE
 141c56e26  mov  r8d, eax
 141c56e2f  call 0x141c57120                 ; (mob, outRect, stance, flag, 0)
```

```asm
FUN_141c57120:
 141c57136  cmp  qword ptr [rcx + 0xe90], r13     ; r13 = 0
 141c5713d  je   0x141c5714a
 141c5713f  cmp  dword ptr [rcx + 0x990], r13d
 141c57146  cmovne ebp, r13d
>141c5714a  lea  rdx, [rcx + 0x42c]               ; <<< the mob's CACHED BODY RECT
 141c5715a  mov  eax, [rdx + 8]
 141c5715d  cmp  [rdx], eax
 141c5715f  jge  0x141c573d7                      ; left >= right   -> BAIL
 141c57165  mov  eax, [rdx + 0xc]
 141c57168  cmp  [rdx + 4], eax
 141c5716b  jge  0x141c573d7                      ; top  >= bottom  -> BAIL
 141c57171  cmp  dword ptr [rcx + 0x98c], r13d
 141c57178  jne  0x141c573d7                      ; mob+0x98c != 0  -> BAIL
 141c5717e  cmp  qword ptr [rcx + 0xa88], r13
 141c57185  je   0x141c5721b                      ; a second bail
...
>141c573d7  mov  qword ptr [rdi], r13             ; <<< writes an ALL-ZERO 16-byte rect
 141c573dc  mov  qword ptr [rdi + 8], r13
 141c573e9  ret
```

**The bail path zeroes the output rect.** It does not signal failure; it hands back a rect that
`141d326ae` will skip. That is the silence. [L]

### 5.3 `mob+0x42c` starts zero - and where the setter actually is

`tools/fieldrefs.py`, **positive control run first and reproduced** (`+0x2f4 --write` over
`0x141c40000..0x141d60000` gives exactly `141c4d261`, `141c4e6ee`, `141cb7ef3`, the three the
gates doc documents): [L]

```
python tools/fieldrefs.py 0x42c --lo 0x141c40000 --hi 0x141d60000 --write
  4 hit(s), 445 resync point(s)
  141c4d427  mov    qword ptr [rsi + 0x42c], r14      in 0x141c4cee0   <- the CONSTRUCTOR
  141c6943f  mov    qword ptr [rdi + 0x42c], r14      in 0x141c68d80   <- a CLEAR (+0x42c/+0x434/+0x43c)
  141c69546  mov    qword ptr [rdi + 0x42c], r14      in 0x141c68d80   <- ditto
  141caff93  movups xmmword ptr [rcx + 0x42c], xmm0   in - (no .pdata entry)
```

`r14` is the mob constructor's zero register (`141c4cf2b xor r14d,r14d`, never reloaded before
`141c4d5eb`) - established in `mob-target-gates.md` §2.3 and reused here. `141c4d427` precedes
`141c4d5eb`, so **a freshly constructed mob has `mob+0x42c` all zero, which is degenerate.** [D]

The one real setter, `141caff93`, is a four-instruction `int3`-padded accessor thunk:

```asm
141caff70  movups xmm0, [rcx+0x42c] / mov rax,rdx / movups [rdx],xmm0 / ret   ; getter
141caff90  movups xmm0, [rdx]       / movups [rcx+0x42c],xmm0 / ret           ; setter
```

and `tools/callers.py` gives **0 call sites** for both - so either they are inlined at every use
(in which case `fieldrefs.py` bounded to the mob range would have found those inlined 16-byte
stores, and it found none) or they are reached indirectly. **`callers.py`'s control is verified
this session** (96 sites / 15 functions for `0x1402fa9a0`, 43 in `0x140304b20`, first
`0x140304e49`, last `0x1403091e7`), so the zero is the tool speaking, not failing. [L]

**RETRACTED, and this is the interesting part.** An earlier draft of this section read
"nothing in the mob code range sets `mob+0x42c`". **That was wrong**, and it was wrong in the
exact way this file had already warned about two sections earlier: **a `[reg+disp]` write-scan
cannot see a store made through a pointer the code `lea`'d and handed to a callee.** Running the
scan *without* `--write` shows three such handoffs, and one of them is the setter:

```
python tools/fieldrefs.py 0x42c --lo 0x141c40000 --hi 0x141d60000     # reads, writes AND lea
  141c5714a  lea rdx, [rcx + 0x42c]   in 0x141c57120   <- a READ use (the degenerate test)
  141ca181d  lea rcx, [r15 + 0x42c]   in 0x141ca16c0   <- a READ use, same shape
  141cb4647  lea rdx, [r15 + 0x42c]   in 0x141cb4600   <- THE SETTER
```

So the correct statement is: **every *direct* write to `mob+0x42c` anywhere in the image is a
zeroing write**, and the single real setter reaches it indirectly. §5.5 follows it. The lesson
is the one `CLAUDE.md` states generally - I wrote the caveat, then drew a conclusion that
ignored my own caveat, and only the follow-up scan caught it. [L]

### 5.4 What this does to `move_action`

`mob-target-gates.md` §2 ends with:

> "So `move_action` round-trips into the animation state and **nothing else**." ... "**`move_action`
> is exonerated.**"

Both halves are correct **about gates 2 and 8**, and §2's CFG-reachability proof for that is
sound and is not disturbed. But the sentence has been read since as a general clearance, and it
is not one: `[mobvtbl+0x68]` reads the animation state, `FUN_141c57120` turns it into the body
rect, and the body rect is the input to the only geometric gate in the collector. **The
animation state is not a dead end; it is the far end of the chain that ends at `141d327c6`.**
[D]

### 5.5 The chain, complete

With the setter found, the whole path from packet to rejection is readable end to end: [L]

```
FUN_141cd1620   141cd1950  mov [r14+0xa88], rax     ; the ONLY setter of mob+0xa88 in mob code.
                                                    ; the ctor leaves it NULL (141c4ddbb, r14=0)
                                        |
FUN_141c68d80   141c6943f/69546  clear mob+0x42c..0x43c
                141c69873/69ada  call FUN_141cb4600 ; clear, then recompute
                                        |
FUN_141cb4600   141cb463b  mov rcx,[r15+0xa88]
                141cb4645  je  0x141cb4680          ; <<< NULL -> RETURN, WRITING NOTHING
                141cb4647  lea rdx,[r15+0x42c]
                141cb4654  call 0x140f82820         ; the body rect is written HERE
                                        |
FUN_141c57120   141c5715f/5716b  mob+0x42c degenerate -> bail 141c573d7 -> ALL-ZERO rect
                141c57178        mob+0x98c != 0     -> bail 141c573d7 -> ALL-ZERO rect
                141c57185        mob+0xa88 == 0     -> bail 141c5721b
                                        |
collector       141d326ae  left >= right -> SKIP the rect (not reject - skip)
                141d327c6  r14b == 0     -> REJECT THE MOB
```

**`mob+0xa88` gates the rect in two independent places** - the setter at `141cb4645` and the
on-demand path at `141c57185` - which is why §7 now watches it rather than only its effect.

Writers of `mob+0xa88` in the mob code range, `fieldrefs.py` control already reproduced:
**two**, the constructor's null at `141c4ddbb` and `141cd1950` in `FUN_141cd1620`. [L]

*Not chased, and flagged rather than guessed:* what makes `FUN_141cd1620` run, and whether our
`0x03C6` reaches it. That is the next question if watch 1 comes back null, and it is a question
about the spawn path rather than the attack path. [I]

---

## 6. The competing candidate: arm C never checks its own attack rect

Worth stating on its own because it is cheap to test and would look identical on screen.

Arm A validates the attack rect before calling the collector: [L]

```asm
1428c2a88  call 0x1401ba9d0        ; skill rect left
1428c2a96  call 0x1401ba9d0        ; skill rect right
1428c2a9b  cmp  ebx, eax
1428c2a9d  jge  0x1428c2cbb        ; left >= right  -> abandon arm A
1428c2aaa/2ab8  ... top vs bottom
1428c2abf  jge  0x1428c2cbb        ; top >= bottom  -> abandon arm A
```

**Arm C has no equivalent.** Between `1428c318c` and `1428c32e9` the rect is built by
`FUN_140ce1950`, offset at `1428c3255..1428c3282`, and clamped only on one side
(`1428c3235 mov r8d,0xffffffbf / cmovl` - a floor of -65). There is no `left < right` or
`top < bottom` test on the arm C path at all. [L]

If the attack rect is degenerate, `141d32713 cmp ecx,edx / jge` fires for every mob rect, `r14b`
stays 0, and `141d327c6` rejects every mob - with healthy mob rects and every documented gate
green. **Symptomatically identical to §5.** The two are distinguished by reading the two
rectangles, which is what §7 proposes. [D]

---

## 7. Ranked, with exact watch addresses

Ranking is by "could reject a snail under arm C", and the top two are chosen so that **one run
separates §5 from §6** rather than confirming one of them.

Both proposed watches sit **inside the collector's per-mob body**, at addresses where `rcx` has
just been loaded with the mob (`141d3265c mov rcx,[rbx+8]`, re-loaded at `141d32671`), so they
fire only while a swing is being processed and `:peek` resolves against the mob.

**These two supersede an earlier pair** (`peek=0x42c` + `peek=0x434`, the rect's two halves).
§5.5 traced the chain one link further back, and reading the *cause* alongside the *effect*
separates three outcomes where reading both halves of the rect separated only two.

| rank | watch | reads | fires |
|---:|---|---|---|
| **1** | **`141d32675:peek=0xa88:hits=40`** | `[mob+0xa88]` - **the animation object**, the root of the chain. Null is the constructor's value | once per mob per swing |
| **2** | **`141d3267c:peek=0x42c:hits=40`** | `[mob+0x42c]` - the cached body rect's **left,top** as one qword | once per mob per swing |

**How to read them.** The pair is diagnostic, not just confirmatory:

* **`0xa88` null** (and `0x42c` therefore zero) -> the mob has no animation object, so
  `FUN_141cb4600` never writes a rect and `FUN_141c57120` bails at `141c5721b`. **The chain in
  §5.5 is the answer end to end**, and the follow-up is `FUN_141cd1620`, the only thing in the
  mob code range that sets `mob+0xa88`.
* **`0xa88` non-null but `0x42c` zero** -> the animation object exists and the rect still was not
  computed. That points at `FUN_141c68d80` (which clears `+0x42c` then calls the setter) never
  having run for our mobs, or at `FUN_140f82820` producing nothing from the animation data.
* **`0xa88` non-null and `0x42c` a sane left/top** -> §5 is dead and **§6 becomes the leading
  candidate**: the mob's geometry is fine, so the intersection is failing on the *attack* rect,
  which arm C never validates.
* **A sane rect** (`left < right`, `top < bottom`, near the mob's screen position) -> §5 is dead
  and **§6 becomes the leading candidate**: the mob's geometry is fine, so the intersection is
  failing on the *attack* rect. The next run reads argument 2's contents; the attack rect's
  address is already known from this capture (`0x146530`, and `[rbp+0x520]` in
  `FUN_1428c1fa0`), and `141d31b20:rdx=` at the collector entry dereferences exactly that
  pointer if `:rdx=` means a dereference - confirm the flag's semantics before spending the slot.
* **A sane rect that does not overlap the player** -> the mob's own idea of where it is has
  drifted from the client's `0x02FF` reports, which would redirect to mob movement, not
  targeting.

**Rank 3, for a later run rather than this one:** gates 18-24, the block entered at
`141d321df` when `FUN_14047c1d0(template)` returns a non-empty list. Seven reject branches, none
of them ever read, and `mob-target-gates.md` §7 item 3 has had it open since. It is ranked below
the geometry only because the geometry explains the symptom exactly and this does not yet
explain anything - but if both rects come back healthy it moves to the top. Watch
`141d321df:peek=0x8` after `141d321d5 call 0x14047c1d0` returns, or simply `141d321e5:hits=40`,
whose mere presence proves the block was entered.

*Not proposed, deliberately:* a watch on `141c57120`. It has three callers and is plausibly
per-frame, so it would flood the log, and it tells you less than reading `mob+0x42c` directly.

---

## 8. Conclusions in `research/mob-target-gates.md` that rest on arm A

Listed for correction, not edited. Numbered so they can be struck one at a time.

| # | where | what it says | status |
|---:|---|---|---|
| 1 | §4.1 heading | "At the **one call site** that uses this collector, `1428c2c2d`" | **wrong** - the client uses `141d25455`, inside `FUN_141d25360`. There are 86 call sites and at least four on an attack path (`research/mob-collector-callsites.md` §3) |
| 2 | §0, §6 | "gates 3, 5, 11, 14, 15, 16 are switched off by the arguments at the only known call site" | **conclusion survives, premise wrong.** They are off under arm C too. §2 |
| 3 | §4 gate 4 row | "arg6 is `[rbp+0x6f80]`, a client field. If non-zero this is a **single-target** filter" | **arm A only.** Arm C passes arg6 = 0, so gate 4 is off. §2 |
| 4 | §4.2, §7 item 4 | "Argument 4 is `esi` in `FUN_1428c1fa0` - a client-side maximum"; "If it is <= 0 the loop never runs" | **arm A only, and now measured on the real path: arg4 = 15.** The early-out is dead as an explanation |
| 5 | §4.2 | "If argument 4 <= argument 17 the loop never starts" | **understates it** - `141d31c96` is the loop header, re-evaluated every iteration (`141d32a62 jmp 0x141d31c90`), and `[rbp+0x620]` is the output cursor, incremented at `141d327de`/`141d32939` |
| 6 | §3 | the three branches at `141d32a18` are "inside a nested container-growth block, plus two list-walk exits" | **wrong** - `141d32692` and `141d327c6` are the geometric gate. §3, §4 |
| 7 | §4 (the whole table) | 17 gates, "argument numbering follows §11.1" | **incomplete** - it contains no geometric test, and the accept it is written against (`141d329f4`) is not the one a plain swing takes (`141d327cc`) |
| 8 | §0, §6 | "static analysis cannot find a per-mob filter that rejects a snail"; "the first gate our mobs fail is: **none of them**" | **a false negative caused by 6 and 7.** There is such a filter |
| 9 | §2, §9 | "`move_action` round-trips into the animation state and **nothing else**"; "**`move_action` is exonerated**"; "no byte in the spawn packet has been found that changes any gate in this chain" | **correct about gates 2 and 8, not a general clearance.** The animation state feeds `[mobvtbl+0x68]` -> `FUN_141c57120` -> the body rect -> `141d327c6`. §5.4 |
| 10 | §6.1 | "it is **not established** that this function runs for a plain 'User Melee' swing" | **closed** - it runs, six for six, from `141d2545a` |
| 11 | §9 | the three-outcome table treats `called-from=0x1428c2c2d` as the "this is the melee builder" case | the measured value is `0x141d2545a`, a fourth outcome the table does not list - and it *does* mean `FUN_1428c1fa0` is the melee builder, just through arm C |

---

## 9. Instrument notes

* **`tools/fieldrefs.py`'s control reproduces** - `+0x2f4 --write` over `0x141c40000..0x141d60000`
  gives exactly `141c4d261`, `141c4e6ee`, `141cb7ef3`. Every `--write` count above was taken
  after that. Note the flag syntax is `--lo`/`--hi`, not positionals; the docstring's own first
  example omits them.
* **`tools/callers.py` was rewritten by another agent while this was being written**, adding
  tail-`jmp` and data-pointer modes. `CLAUDE.md` says every conclusion drawn with a changed
  instrument is suspect **including your own**, so everything above that used it was re-run
  against the new version. All of it holds, and two results got *stronger*:
  - `0x141d31b20`: **86 call sites, 0 tail jmps, 0 qword pointers** - the return-address table in
    `research/mob-collector-callsites.md` §3 is intact, and the new tool's independent pointer
    scan agrees with the one written for that file.
  - `0x141c88970`: 1 call site, 0 tail jmps, 0 pointers. Sole caller `141d33158` confirmed.
  - `0x141c56e00`: 0 calls, 0 tail jmps, **1 qword pointer, at `0x1434077f8`** - which is exactly
    the `mobvtbl + 0x10` this file derived at §5.1 from an unrelated anchor. Two independent
    routes to the same slot. (The tool notes the surrounding pointer run is
    `0x1434070f0..0x143407a88` with no RTTI locator at its head, so the *run head* is not the
    vtable base; the §5.1 anchor does not depend on it.)
  - `0x141caff70` / `0x141caff90`: 0 calls, 0 tail jmps, **0 pointers**. Genuinely unreachable as
    functions, so the `mob+0x42c` accessors are inlined at every use - which is what makes §5.3's
    "nothing in the mob code range sets it" a real result rather than a missed indirect call.
* **`tools/callers.py`'s control reproduces on both versions** - 96/15 for `0x1402fa9a0`, 43 in
  `0x140304b20`. Every zero reported here was taken after it.
* **The reject-branch enumeration was re-derived, not inherited**, by bucketing every branch in
  the extent by target with no assumption about which labels matter. It independently produced
  §3's 27, including the `141d32a06` exclusion that file already documents.
* **The vtable read is anchored, not guessed** - `mobvtbl` was derived from a documented slot
  (`+0xd0 -> FUN_141cbcad0`) and the anchor reproduces when read back. `/OPT:ICF` means this is
  [D], not [L].
* **`raw` capstone dumps silently return nothing when the start lands mid-instruction.** Two dumps
  in this session came back empty and looked like "there is no code there"; both needed a resync
  sweep over candidate start offsets. `0x141caff93` sits past the end of the nearest `.pdata`
  entry, which `docs/ghidra.md` calls the normal state for interesting code.
* **The whole-image `fieldrefs --write` sweep for `+0xe78` finished** (666 939 resync points, so it
  really did cover `.text` and `.boot`) and returned **8 hits**. Seven are outside the mob code
  range, in the `0x1407`/`0x1408`/`0x140f` bands, and five of those are **dword** stores that
  cannot be writing a pointer at all. The two qword ones - `140f7e4c7` in `0x140f7dcf0` and
  `140fad64e` in `0x140fad630` - have callers only in the `0x140d`/`0x1410`/`0x1411`/`0x1414`
  UI/animation bands, never in mob code, so they are a **different class's field at the same
  displacement** - the exact trap `fieldrefs.py`'s own docstring names ("a displacement is a
  class fact, not an offset fact"). **The only write to `mob+0xe78` in the whole image is the
  constructor's zero at `141c4e23d`.**
  The reason that is not a contradiction: `FUN_141c88970` allocates the array through a `lea`'d
  pointer (`141c88a76 lea rsi,[r15+0xe78]`), and **a `[reg+disp]` write-scan structurally cannot
  see a store made through a pointer it handed off.** So the sweep's emptiness is a property of
  the instrument, and `FUN_141c88970` - sole caller `141d33158`, one arm of the `0x03D9..0x044D`
  per-mob command switch - remains the only thing that fills it.
* **The whole-image sweep for `+0x42c` finished** and returned 29 hits. Only four are qword or
  xmmword and so could touch a rect: `141c4d427` (ctor), `141c6943f`/`141c69546`
  (`FUN_141c68d80`), `141fdb9b5` (`FUN_141fdb720`) - **all four are zeroing writes**, confirmed
  by their shape (runs of stores across `+0x42c`/`+0x434`/`+0x43c`/`+0x444`) and by their source
  register being a zeroed one (`141c4cf2b xor r14d,r14d`, `141fdb788 xor r15d,r15d`). The
  remaining 25 are **dword** stores in the `0x141e4`/`0x141e5`/`0x1420a-e`/`0x14216`/`0x14233`/
  `0x142e5` bands - different classes at the same displacement.
  `141fdb9b5` is worth one line because it initially looked like a rival setter outside the mob
  code range: it is not, and `141fdb99d mov rax,[rbx+0x3a8]` two instructions earlier - the
  template pointer, per `mob-target-gates.md` §4.3 - is what proves `rbx` really is a mob there.
* **That sweep is also what caught the retraction in §5.3.** The `--write` scans, mob-range and
  whole-image alike, agreed that every direct write is a clear; both were structurally blind to
  `141cb4647`, the real setter, because it stores through a `lea`'d pointer. **Two independent
  scans agreeing is not corroboration when they share a blind spot** - which is the same failure
  mode `tools/dataref.py`'s docstring records ("two independent filters, both dropping the same
  evidence, both silently"). Dropping `--write` found it in one call.
* **No client run. No Ghidra** - the lock was held but nothing here needed it; everything is
  capstone through the repo's own loaders, run with the repo as the working directory.
