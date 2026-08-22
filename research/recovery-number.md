# The blue recovery number — why `0x007C`'s recovery trailer drew nothing, and what does draw it

The owner, 2026-08-21: *"the idle recovery should pop up with a blue number of the recovery amount
above the player's head. I don't see that here, the HP bar just moves up without a number
indication."*

The measurement that started this is
`research/fixtures/skill-count-ignored-and-recovery-trailer-sent.log`: three `idle regen …
with the recovery trailer` lines, the bar moved on all three, no number on any of them. A
clean negative — the packet arrived, was well-formed, and drew nothing.

Evidence tags: **[L]** read off the listing, **[D]** deduced from code that could have
disagreed, **[I]** inferred / named by analogy.

---

## 0. The three answers, up front

1. **The `oldHp`/`oldMp` hypothesis is dead.** The snapshots at `142d5485c` and `142d5486b`
   are taken **65 bytes before** the mask block runs at `142d5489d`. They really are the old
   values. Sending the recovery trailer without the hp/mp mask bits would have changed
   nothing. **[L]**
2. **`FUN_140fd31f0` is not a drawing function and never was.** It is an accounting object:
   it adds the recovery to running totals, clamps each against the headroom
   `maxHp - oldHp` to separate *effective* from *wasted* recovery, keeps per-hour averages,
   and resets itself after 3 600 000 ms. It contains **no draw call of any kind**. The
   `0x007C` recovery trailer therefore *cannot* put a number on screen, no matter what is
   in it or what else the packet carries. **[L]**
3. **The blue number is `FUN_142771360(pUser, +N, …)`** — positive picks the `NoBlue` digit
   set, negative picks `NoViolet`, and that one function is the only site in the whole image
   that ever asks for `NoBlue`. Three packets can reach it with a positive `N`, all on the
   local-user path: **`0x02D1` effect `0x41`** (`i32 amount; i32 delayMs; i32 id` — the server
   picks the number), **`0x02D1` effect `0x23`** (`i32 A; u8 flag; i32 B`), and **`0x02F2`**
   (a `u32` key, with the number read out of the WZ `spec` node — the potion effect, no good
   for regen). `0x02AF` + a leading `u32 charId` does any of it over another player. **[L]**

§8 is the wire-up. **`0x41` is the recommendation**; §8.2 says why, and §7 is the one gate
that could still make all of this draw nothing.

---

## 1. The order of operations — the hypothesis, killed

The claim to test was: *the `oldHp`/`oldMp` arguments are snapshotted after the mask block has
already stored the new totals, so the delta is zero.* It is wrong, and the listing says so
without ambiguity.

`FUN_142d54780`, in address order (`research/msexe-statchange-handler.txt`):

```
142d547f4  call 0x142cbe730          ; rdi = the character-stat record
142d547f9  mov  rdi, rax
...
142d5485c  lea  rcx,[rdi + 0x5b]     ; hp
142d54860  mov  edx,[rdi + 0x63]     ;   its check word
142d54863  call 0x1401ba9d0          ;   the obfuscated getter
142d54868  mov  [rbp - 0x58], eax    ; <-- oldHp SAVED HERE
142d5486b  lea  rcx,[rdi + 0x73]     ; mp
142d5486f  mov  edx,[rdi + 0x7b]
142d54872  call 0x1401ba9d0
142d54877  mov  r15d, eax            ; <-- oldMp SAVED HERE
...
142d5489a  mov  rcx, rdi
142d5489d  call 0x1402cbb50          ; <-- THE MASK BLOCK. 65 bytes later.
142d548a2  mov  edi, eax             ; the mask, kept
142d548aa  READ u8                   ; trailer 1 flag (charm)
142d548d2  READ u8                   ; trailer 2 flag (recovery)
142d548de  READ u32 -> edi           ; hpRecovery
142d548e8  READ u32 -> ebx           ; mpRecovery
142d548fc  mov  [rsp + 0x20], r15d   ; arg5 = oldMp   (the snapshot)
142d54901  mov  r9d, [rbp - 0x58]    ; arg4 = oldHp   (the snapshot)
142d54905  mov  r8d, ebx             ; arg3 = mpRecovery
142d54908  mov  edx, edi             ; arg2 = hpRecovery
142d5490a  mov  rcx, [rip + 0xd749b7]; arg1 = a global object
142d54911  call 0x140fd31f0
```

**[L]** The snapshots are pre-decode. `[rbp-0x58]` sits at `rsp+0xC8` inside this function's
own 0x320-byte frame, above the 0x20-byte outgoing home area, so `FUN_1402cbb50` cannot
clobber it; `r15` is callee-saved and `FUN_1402cbb50` must preserve it. `oldHp != newHp`
whenever bit 10 carried a new total.

So the fix the hypothesis implied — send the trailer *without* the hp/mp mask bits — would
have produced exactly the same blank screen, for a completely different reason. §2.

> The `0x007C` handler's decompiled text would have been no help here either way. This is the
> case `docs/ghidra.md` means by *field order from the listing*: two `call 0x1401ba9d0`s and a
> `call 0x1402cbb50` reorder freely in decompiled C and not at all in the listing.

---

## 2. What `FUN_140fd31f0` actually does with its five arguments

`0x140fd31f0 .. 0x140fd3573`, 899 bytes, 6 contiguous `.pdata` entries merged
(`research/msexe-recovery-popup.txt` — the filename is now a misnomer and is kept only
because the listing inside it is what settles this).

**Its complete call list**, which is the whole argument:

| at | call | what |
|---|---|---|
| `140fd3218` | `[rip+0x228fb92]` | a tick |
| `140fd3241` | tail `jmp 0x140fd2490` | the hour-rollover path |
| `140fd3293` | `0x142cbec90` | fetch an object |
| `140fd32a2`, `140fd32c1` | `0x1401ba9d0` | the obfuscated getter, twice |
| `140fd343a` | `[rip+0x228f970]` | another tick |

**[L] There is no fifth kind. Nothing here draws, loads a canvas, touches a UI pool, or
formats a string.** That is the finding; everything below is what it does instead.

```
140fd31fb  mov  eax,[rcx + 0x18]
140fd320c  je   → return                 ; a master "enabled" flag
140fd3212  cmp  [rcx + 0x20], 0
140fd3218  call [rip + 0x228fb92]        ; tick
140fd321e  sub  eax,[rdi + 0x168]        ; since the window started
140fd3224  cmp  eax, 0x36ee80            ; 3,600,000 ms == one hour
140fd3229  jle  140fd3246
140fd3241  jmp  0x140fd2490              ; roll the window over

140fd325b  add  [rdi + 0x208], r15       ; += hpRecovery   (raw, as sent)
140fd3262  add  [rdi + 0x210], rsi       ; += mpRecovery   (raw, as sent)

140fd3293  call 0x142cbec90              ; rbx = the object that holds maxHp/maxMp
140fd32a2  call 0x1401ba9d0              ; maxHp  <- obf triple at +0x60 / check +0x68
140fd32b4  sub  r12d, r13d               ;   r13d = arg4 = oldHp
140fd32bd  cmovs r12d, 0                 ;   headroomHp = max(maxHp - oldHp, 0)
140fd32c1  call 0x1401ba9d0              ; maxMp  <- obf triple at +0x78 / check +0x80
140fd32cd  sub  ecx,[rsp + 0x70]         ;   [rsp+0x70] = arg5 = oldMp
140fd32d5  cmp  r12d, r15d               ; headroomHp vs hpRecovery
140fd32dd  add  [rdi + 0x1f8], r14       ; += min(headroomHp, hpRecovery)  <- EFFECTIVE
140fd32f0  add  [rdi + 0x200], rbp       ; the same for mp
```

then event counters at `+0x228/+0x22c/+0x230/+0x234`, per-event averages at
`+0x238..+0x244` (`idiv` by the counts), percentages at `+0x218..+0x224` (`imul …, 0x64`
then `idiv`), a smoothed rate at `+0x250/+0x258`, and a saturation check against
`0x7ce66c50e2840000` that wipes all thirteen fields when any total gets that large.

**[L]** raw-vs-effective split, the one-hour window, the absence of any draw.
**[I]** the name. An object that measures *how much of your healing was wasted* per hour, and
whose hour-rollover (`FUN_140fd2490`) formats a message, is a play-summary / over-play-time
reporter. `Effect/BasicEff.img/OverPlayTime/start` exists in this image, which is suggestive
and no more. **The name does not matter for the conclusion**: what matters is the empty
column above.

### `0x143AC92C8` is one global, shared with the EXP path

`tools/dataref.py 0x143AC92C8` — 139 references, one write at `0x140fd1e6d` (the
constructor), and among the reads:

```
read   at 0x142d5490a   in 0x142d54780     ; arg1 of FUN_140fd31f0   (recovery)
cmp    at 0x142d54925   in 0x142d54780     ; the null-check on the EXP path
read   at 0x142d54993   in 0x142d54780     ; arg1 of FUN_140fd3110   (exp)
```

That was read off the tool, not computed by hand — the first hand-computed value for this
global was wrong by 0x600.

Note the asymmetry, because it is a real hazard: the **EXP** path null-checks the global
before using it (`142d54925`), the **recovery** path does not (`142d5490a` loads it straight
into `rcx`). If that global is ever null when a recovery trailer arrives, `FUN_140fd31f0`
dereferences it at `140fd31fb` before doing anything else. On the three logged ticks it
plainly was not null. **[L]** for the asymmetry, **[D]** for it being a live hazard.

---

## 3. A correction I owe `research/level-up.md` §5

§5 reads:

```
142d5499a  call 0x140fd3110               ; show "+N EXP"
```

`FUN_140fd3110` is 106 bytes, `0x140fd3110 .. 0x140fd317a`, and in full it is:

```
140fd311a  mov  eax,[rcx + 0x18]     ; the same enabled flag
140fd3125  je   → return
140fd312d  call [rip + 0x228fc7d]    ; tick
140fd3139  cmp  eax, 0x36ee80        ; the same one-hour window
140fd3156  jmp  0x140fd2490          ; the same rollover
140fd3168  add  [rbx + 0x158], rdi   ; <-- totalExpGained += delta.  That is all.
```

`rcx` is `[rip+0xd7492e]` = **`0x143AC92C8`, the same accounting object**. It draws nothing.

**[L] `FUN_140fd3110` is an accumulator, not an indicator.** §5's *"The EXP indicator is
client-side too"* rests on it, and on that function it is wrong. The arithmetic §5 describes
(`newExp - oldExp`, plus `expNeededForLevel(oldLevel)` when the level moved) is real and is
still the best evidence for naming bit 16 `exp` — the *consumer* is simply a counter rather
than a UI element.

I did **not** look for an EXP indicator elsewhere in the client, so this retracts one
sentence and establishes nothing about whether "+N EXP" appears from some other route. Blind
spot, stated plainly: I only followed the calls that leave `FUN_142d54780`'s bit-16 block.

---

## 4. Where a blue number actually comes from

`FUN_142771360` (`0x142771360 .. 0x142771939`, `research/msexe-shownumber.txt`) is the
floating-number-over-a-user function. Signature, from its callers and its own use:

```
FUN_142771360(CUser *pUser, int nAmount, int nOffsetIndex, int, int, int)
```

It forks on the **sign** of `nAmount`:

```
142771393  test edx, edx
142771395  jns  1427713ad
142771397  call 0x1429e3ef0              ; negative only: stamp "was damaged"
14277139c  mov  [r12 + 0x34f8], eax
1427713a4  mov  byte [r12 + 0x34fc], 1
...
142771489  test edi, edi
14277148b  jle  142771568                ; <= 0 goes elsewhere
   POSITIVE:
14277154e  mov  dword [rsp + 0x20], 2    ; <- digit set 2
14277155e  call 0x140e0ccb0
   NEGATIVE (142771568 jns → 14277166a for exactly zero):
1427715ef  neg  edi                      ; magnitude
142771629  mov  dword [rsp + 0x20], 3    ; <- digit set 3
142771637  call 0x140e0ccb0
```

That `2`/`3` is threaded through three frames to a jump table, and each hop was checked:

| hop | at | what moves |
|---|---|---|
| `FUN_142771360` → `FUN_140e0ccb0` | `14277154e` / `142771629` | written to outgoing arg5 |
| `FUN_140e0ccb0` → `FUN_140e685e0` | `140e0cd02  mov r9d,[rsp+0xc0]` | arg5 becomes arg4 |
| inside `FUN_140e685e0` | `140e6860f  mov r12d, r9d` | arg4 → `r12d` |
| `FUN_140e685e0` → `FUN_140ee8f00` | `140e69329  mov [rsp+0x20], r12d` | `r12d` becomes arg5 |
| `FUN_140ee8f00` | `140ee8f79 movsxd rax,[rsp+0x180]` / `140ee8f81 cmp eax,5` | arg5 indexes the table at `0x140ee97cc` |

The table, dereferenced (`tools/dump_va.py 0x140ee97cc 32`) and each `lea` target resolved:

| index | block | canvases |
|---:|---|---|
| 0 | `140ee8f9d` | `NoRed0`, `NoRed1`, `NoCri0`, `NoCri1` |
| 1 | `140ee8fc2` | `NoRed2`, `NoRed3`, `NoCri2`, `NoCri3` |
| **2** | `140ee8fe7` | **`NoBlue0` (`0x1432b0c08`), `NoBlue1` (`0x1432b0c18`)** |
| 3 | `140ee8ff7` | `NoViolet0` (`0x1432b0c28`), `NoViolet1` (`0x1432b0c40`) |
| 4, 5 | `140ee9007` | `NoProduction0`, `NoProduction1` |

**[L] A positive amount draws in `NoBlue`. A negative amount draws in `NoViolet`.** That is
the blue number the owner is describing, and the fork is one `test`/`jle`.

### And `FUN_142771360` is the *only* thing that ever asks for set 2

`tools/callers.py 0x140e0ccb0` — **11 call sites in 10 functions**. The digit-set selector is
written to the outgoing `[rsp+0x20]` at every one of them, and all eleven were read:

| selector | sites |
|---:|---|
| 0 | `141c99112` (`xor ecx,ecx` at `141c990d8`), `14199836a` (`r13d`, zeroed at entry), `14291e23e` (`xor edx,edx` at `14291e20a`) |
| **2** | **`14277154e` — `FUN_142771360`, positive branch. One site in the image.** |
| 3 | `14198d7ed`, `14198dab7`, `1420d2360`, `142771629` |
| 4 | `142798e77`, `142799098` |
| 5 | `1423f0753` |

**[L]** Nothing else in the client can put a blue digit on screen. Three of the eleven pass a
register rather than a constant and all three were traced to a zero in the same basic block —
that is the part a sampled read would have got wrong.

---

## 5. Every producer of a positive amount, enumerated — properly, the second time

`tools/callers.py 0x142771360` — **30 call sites in 23 functions**, plus 0 tail jumps and 0
data pointers. (Control first: `tools/callers.py 0x1402fa9a0` gave the documented 96 sites,
43 of them in `0x140304b20`.)

> **The first pass of this section was wrong, and the way it was wrong is worth writing down.**
> I built the site list by hand from `callers.py`'s per-function summary, which prints `first`
> and `last` per function and not the sites in between. Four of the 23 callers have more than
> one site. **Three sites of thirty were never looked at** — `14278e261` (the middle of
> `FUN_1427863f0`'s three) and both of `FUN_1428ee7d0`'s, a function that dropped out of the
> hand-built list entirely — and **two of those three are positive**, which is to say they are
> two of the three answers this document exists to give. `callers.py` was right; the reading of
> its output was a sample dressed as an enumeration, and it was labelled *"I read the operand
> feeding `edx` at all 30, not a sample"*. Redone mechanically: walk each of the 23 functions'
> listings for *every* `call 0x142771360`, and assert the count comes back to 30.

**Twenty-three of the thirty execute `neg` on the taken path** — plain damage. The seven that
do not, all of them:

| site | in | value in `edx` | sign |
|---|---|---|---|
| `142790d67` | `FUN_1427863f0` | `xor edx, edx` | always 0 — the MISS/guard path |
| `1428aca14` | `FUN_1428aa0a0` | `edx = ebx - r12d`, where `1428ac91a..1428ac922` make `ebx = min(r12d, r12d - eax)` | `-max(eax,0)`, **≤ 0 for every input** |
| `1428eea99` | `FUN_1428ee7d0`, the `edi < 0` arm of `1428eea82 jns` | `edi`, verbatim | negative by construction |
| `14278d89e` | `FUN_1427863f0`, effect 6/10 | a `u32` from the packet, verbatim | **can be positive**, but gated on `14278d875 cmp ebx, 0x3eba98` (4 111 000) — a hardcoded id, not a general route |
| `14278e261` | `FUN_1427863f0`, **effect `0x23`** | a `u32` from the packet, verbatim | **can be positive** — §6.2 |
| `1428eeb0e` | `FUN_1428ee7d0`, the `edi > 0` arm, fed by **opcode `0x02F2`** | `edi`, verbatim, read out of WZ data | **can be positive** — §6.3 |
| `14281e439` | `FUN_14281e390` | straight out of a queue node | **can be positive** — §6.1 |

`14278e261` is the one an automated "is there a `neg` in the preceding N instructions" filter
still gets wrong: there *is* a `neg esi` at `14278e234`, on the other arm of the branch. It
has to be read, not grepped.

**`0x14281e439`, in `FUN_14281e390`, passes the value straight out of a queue node with no
arithmetic and no gate at all.** That is the route §8 recommends.

```
14281e415  mov  edx, r15d            ; the current tick
14281e418  mov  ecx, [rdi + 4]       ; the node's due tick
14281e41b  call 0x1408fc980          ; = (tick - due) > 0
14281e422  je   → skip
14281e434  mov  edx, [rdi]           ; <- the amount, verbatim
14281e439  call 0x142771360
14281e43e  mov  edx, [rdi + 8]       ; the node's id
14281e443  sub  ecx, 0x13da0e        ; only 0x13DA0E / 0x13DA0F get an extra effect
```

`FUN_1408fc980` is four instructions — `sub edx,ecx; test edx,edx; setg al; ret` — so a node
fires on the first `CUser::Update` strictly **after** its due tick. `FUN_14281e390` is called
from `FUN_14279c480` (`CUser::Update`) at `0x1427a0139`, with the tick in `edx`.

### The blind spot that nearly hid the producer, and how it was got round

`tools/rangescan.py 0x39b8 0x140001000 0x143000000` returns 35 sites and **not one of them
enqueues**. The list at `pUser+0x39b8` looked drain-only. That answer is clean, confident and
wrong in the exact way `CLAUDE.md` describes for `mob+0x42c`: `rangescan` inspects **memory
operands with a matching displacement**, and the enqueue does

```
14281e2fe  add rsi, 0x39b8
```

— an **immediate**, not a displacement. Re-running `rangescan` over a wider range would never
have found it; it is structurally invisible to that tool. Changing the question did: a
one-off pass that greps the *rendered operand text* of every instruction (superset of
`rangescan`; positive control `0x2f4` → `141cb7ecb` and `141cb7ef3` in `FUN_141cb6880`)
returned 115 sites, and `14281e2fe` is in the difference.

---

## 6. The three packets that can put a positive number over a character

### 6.1 `0x02D1` effect `0x41` — an amount, a delay, an id  ← use this one

```
FUN_14281e2d0(CUser *pUser, i32 nAmount, i32 nDelayMs, i32 nId)     0x14281e2d0..0x14281e37d

14281e2ef  call 0x1429e3ef0            ; tick
14281e2f4  mov  [rsp+0x20], ebx         ; nAmount
14281e2f8  add  eax, edi                ; tick + nDelayMs
14281e2fa  mov  [rsp+0x24], eax
14281e2fe  add  rsi, 0x39b8             ; the std::list on the user
14281e326  mov  edx, 0x20               ; a 0x20-byte node
14281e33d  movsd [rax+0x10], xmm0       ; node+0x10 = amount, node+0x14 = due tick
14281e342  mov  [rax+0x18], ebp         ; node+0x18 = id
14281e345  inc  qword [rsi + 8]         ; _Mysize
```

Node layout matches the drain exactly (`lea rdi,[rbx+0x10]`; `[rdi]`, `[rdi+4]`, `[rdi+8]`).

`tools/callers.py 0x14281e2d0` — **exactly one call site in the whole image**, no tail jumps,
no data pointers:

```
0x142790848   in FUN_1427863f0        ; the UserEffect handler
```

and the block it is in:

```
142790821  READ u32 -> edi     ; arg2  nAmount
14279082e  READ u32 -> ebx     ; arg3  nDelayMs
142790838  READ u32 -> r9d     ; arg4  nId
142790845  mov rcx, r15        ; arg1  the user the effect packet named
142790848  call 0x14281e2d0
```

`tools/reads.py 0x1427863f0 1` — the ten-primitive walker — reports those three and only
those three at `0x142790824`, `0x14279082e`, `0x142790838`, and reports **no reads at all**
inside `FUN_14281e2d0` or `FUN_14281e390` at depth 3. So the body is exactly 12 bytes after
the effect byte. (`reads.py`'s own control, `0x140304100` at depth 2, showed the mixed
direct/helper output its docstring requires before I believed any of this.)

#### Which effect type

`FUN_1427863f0` dispatches twice. The first switch (`1427864a8 cmp ecx,0x45`, byte-index
table `0x142791300`, target table `0x14279129C`, `ecx = type - 8`) sends type 65 to its
`default`. The `default` at `0x14278bd20` runs a second switch over the **raw** type
(`14278bd7d cmp ebx,0x54`, table `0x142791348`, 85 entries). Resolving `0x142790848` against
that table gives block `0x142790821`, whose only key is:

> ### effect type `0x41` (65)

Corroboration on the same tables: entry `[0]` is `0x14278bd99`, the block that reads
`0x143a46f48` at `0x14278bdc8` — `Effect/BasicEff.img/LevelUp`, which `research/level-up.md`
§5 established independently. And resolving the local-user dispatch table `0x14289d660`
against the effect-handler stub `0x14289a439` returns exactly one opcode, `0x2d1`, matching
`net::stats::USER_EFFECT_LOCAL`.

### 6.2 `0x02D1` effect `0x23` — an amount, a flag, a second amount

The same handler, block `0x14278e204`, second table key **`0x23` (35)**. Body, widths from
`tools/reads.py 0x1427863f0 1`:

```
14278e207  READ u32 -> esi     ; A
14278e211  READ u8  -> ebx     ; flag
14278e21c  READ u32 -> edi     ; B
14278e223  test ebx, ebx
14278e225  je   14278e24c                      ; flag == 0  ->  straight to the number
14278e22d  call [vtable + 0x50] on the user    ; flag != 0  ->  a predicate on the user
14278e232  je   14278e24c                      ;   false    ->  same place
14278e234  neg  esi
14278e245  call 0x14291e000                    ;   true     ->  the OTHER renderer, digit set 0
>14278e24c
14278e25c  mov  edx, esi                       ; A, verbatim -> positive draws NoBlue
14278e261  call 0x142771360
>14278e266  neg  edi
14278e27a  jle  → done                         ; B must have been NEGATIVE to go further
14278e2c7  call 0x1428d93b0(global, -B)
```

**[L]** `A = +N, flag = 0, B = 0` draws `N` in blue immediately and does nothing else: the
`flag` test skips the vtable predicate entirely, and `B = 0` fails the `jle` at `14278e27a`.
Nine bytes after the effect byte instead of twelve, and no queue.

**[D]** I did not establish what `flag` or `B` mean, and `flag != 0` routes to a completely
different renderer with a completely different digit set. §8 recommends `0x41` over this for
exactly that reason — `0x41`'s three fields have no branch on any of them.

### 6.3 `0x02F2` — an item id, and the client looks the amount up itself

`FUN_1428ee7d0` (`0x1428ee7d0..0x1428eeb7a`, `research/msexe-incdec-number.txt`) is reached
from the local-user dispatcher `FUN_14289a3a0` at `0x14289ad10`, and resolving that against
the table at `0x14289d660` gives exactly one opcode: **`0x02F2`**. `tools/callers.py` finds no
other way in, and `tools/reads.py 0x1428ee7d0 3` finds **one** read in the whole subtree:

```
1428ee7f0  READ u32 -> eax
1428ee7fc  mov  rcx,[rip + 0x11b9b25]
1428ee803  call 0x14039e630          ; look eax up in a data table; null -> return
1428ee816  mov  rdi,[rip + 0x1157c2b] ; -> 0x1432abb60, the UTF-16 string "spec"
...
1428ee9ca  movzx eax, word [rbp - 0x11]
1428ee9ce  cmp  ax, 3                ; VT_I4
1428ee9d4  mov  edi,[rbp - 9]        ; <- the amount, out of the WZ property
...
1428eea80  test edi, edi
1428eea82  jns  1428eeaf7
1428eea94  mov  edx, edi   ; < 0 -> FUN_142771360 -> NoViolet
1428eeaf7  jle  → nothing  ; == 0 -> nothing at all
1428eeb09  mov  edx, edi   ; > 0 -> FUN_142771360 -> NoBlue
```

**[L] The `u32` is a key, not an amount.** The number drawn comes from the `spec` node of
whatever that key names — the shape of a consumable's `spec/hp` / `spec/mp`. **[I]** that it
is an item id; the lookup table at `0x143AA8328` was not identified.

So `0x02F2` is the right packet for *"the player drank potion X"* and the wrong one for idle
regen: the server does not choose the number, `Item.wz` does. Worth wiring separately when
potions get their on-screen effect.

---

## 7. The one gate I could not prove passes — read this before spending a launch

Between the second switch's entry and the jump, the `default` block has a suppression test
that types `0x4F`, `0x50` and `0x51` skip and **types `0x41` and `0x23` do not** (both are
`0x18` — the `default` — in the first switch's byte-index table at `0x142791300`, so both
arrive here):

```
14278bd29  call 0x141892840          ; the current CField, or null
14278bd3b  call 0x14182ffd0          ; = *(u8*)(field->[+0xa8] + 0x2d9)   -> bl
14278bd45  mov  bl, 1                ;   (no field  ->  bl = 1)
14278bd4a  call 0x142826340          ; on the user: a compound "not renderable" test
14278bd4f  jne  14278bd5b            ;   true  -> dl = 0
14278bd53  test bl, bl
14278bd55  jne  14278bd5b            ;   true  -> dl = 0
14278bd57  mov  dl, 1
14278bd75  test dl, dl
14278bd77  je   → discard the packet
```

**[D]** `FUN_142826340` is a hide/visibility predicate (it chains `0x141715f80`, a
vtable `+0x50` call, `0x140fb0030`, and a `+0xd0` vtable call on the user's `+0x100`
sub-object). `FUN_14182ffd0` is a single flag byte on a field sub-object — a per-map
"effects off" option. Both should be 0 for a visible character standing on an ordinary map.
**Neither is measured.** If effect `0x41` produces nothing on screen and the hook log shows
`0x02D1` dispatched and returned, this gate is where to look first — not the packet body.

And because `0x23` sits behind the *same* gate, **swapping `0x41` for `0x23` is not a test of
this gate**. If it has to be tested, the cheap way is a type that skips it: `0x4F`, `0x50` or
`0x51` reach the jump table unconditionally. Whether any of those three is safe to send is not
established here.

`FUN_142771360` then has four more early-outs of its own
(`1427713b4`, `1427713c8`, `1427713ec`, `142771431`) before it draws. The LevelUp effect at
`0x14278bd99` passes the same first two globals, which is weak corroboration only — the
level-up animation the owner has seen is played by the `0x007C` handler at `142d54bdb`, not
through here.

---

## 8. WIRE IT LIKE THIS

An idle-regen tick that heals `n` HP becomes **two** packets, and the second one is new.

### 8.1 `0x007C` — unchanged, keep sending what already works

The bar movement is already correct. Send the new **totals**, exactly as now:

```
u8   bExclRequestSent   0
u8   ?                  0
u8   ?                  1
u32  mask               0x00400 for hp alone  (| 0x01000 if mp also moved)
u32  hp                 the NEW total          ; bit 10, only if set
u32  mp                 the NEW total          ; bit 12, only if set
u8   hasCharm           0
u8   hasRecovery        0        <- see below
```

**Set `hasRecovery` to `0`.** `StatChange::recovery = None`. The two `u32`s behind that flag
reach `FUN_140fd31f0` and nothing else, and `FUN_140fd31f0` draws nothing (§2). Sending them
is not *wrong* — the client's own accounting is slightly more accurate with them — but it is
nine bytes that can never appear on screen, and leaving the flag set invites the same wrong
conclusion again. If they are kept, they must stay as they are: **amounts gained**, beside
`hp`/`mp` carrying **new totals**, because `FUN_140fd31f0` computes `maxHp - oldHp` from its
own pre-decode snapshot and adds the raw amount separately.

### 8.2 `0x02D1` — the packet that draws the number

```
u16  opcode      0x02D1        // net::stats::USER_EFFECT_LOCAL
u8   effect      0x41          // 65
i32  amount      n             // POSITIVE -> NoBlue.  Negative would draw NoViolet.
i32  delayMs     0             // ms from now; fires on the first CUser::Update after it
i32  id          0             // only 0x13DA0E / 0x13DA0F add an extra animation
```

13 bytes after the opcode. Nothing else follows; the handler reads exactly three `u32`s in
this case and returns.

Concretely, healing 10 HP: `D1 02 41 0A 00 00 00 00 00 00 00 00 00 00 00`
(opcode LE, effect, amount=10, delay=0, id=0).

**Order:** send `0x007C` first, then `0x02D1`. Nothing in the client couples them — the
number comes from a queue on the user object, the bar from the stat record — but a number
that appears before the bar moves would read as a glitch.

**HP and MP together:** the packet carries one number, so send two `0x02D1`s. Give the second
a `delayMs` of ~400 so they do not stack on the same pixel. Both will be blue; the client has
one positive digit set.

**Other players:** the identical effect over a remote character is `0x02AF` with a `u32
charId` in front of the effect byte — `FUN_1429bb720` reads that id at `0x1429bb745` (its
only direct read) before dispatching into the same `FUN_1427863f0`. Idle regen is probably
not worth broadcasting; potions and heals are.

**Why `0x41` and not `0x23`.** `0x23` (§6.2) draws the same blue number from a shorter body
and without the queue, and `A = +n, flag = 0, B = 0` is measured to be inert in every other
respect. It is a legitimate second variant. `0x41` is the recommendation only because all
three of its fields are plain ints that nothing branches on, whereas `0x23`'s `flag` selects
between two different renderers and its `B` has a sign precondition — two more ways for a
first attempt to come back ambiguous. **Test one at a time**; do not send both.

**And not `0x02F2`.** §6.3: its `u32` is a lookup key and the number comes out of `spec` in
the WZ, so the server cannot choose the amount. That is the packet for potions, later.

### 8.3 What to watch, and what each outcome means

One variant per launch. `0x02D1`/`0x41` alone, on top of the `0x007C` that already works:

| on screen | reading |
|---|---|
| a **blue** number equal to the heal, above the head, rising and fading | done — §6.1 confirmed end to end, statically |
| a **violet** number | `amount` went out negative. It is read as a signed `int` at `142771489` |
| nothing, and `client-patched\maplecw-hook.log` has **no** dispatch line for `0x02D1` | the packet never arrived or never dispatched — a framing problem, not this |
| nothing, and the hook log **has** a dispatch line that returned | the §7 gate at `14278bd75`, or one of `FUN_142771360`'s four early-outs. Not the body |
| the client freezes | not this packet: it is server→client and needs no reply, and the handler makes no allocation past the 0x20-byte node |

The dispatch line is written on **return**, so a *missing* line with the packet present in
`world.log` means the handler was entered and did not come back — count it in both files.

---

## 9. What I did NOT establish

* **That `0x41` is what Nexon's server uses for idle regen.** What is established is that it
  is one of three server-reachable routes to a positive (blue) number over a user, enumerated
  over all 30 call sites of `FUN_142771360` and all 11 of `FUN_140e0ccb0`. The effect *is* the
  thing the owner described; which of the three the real server used is not knowable from this
  binary, and it does not have to be to make the screen right.
* **Whether the §7 suppression gate passes in normal play.** Named, not measured, and it
  covers `0x41` and `0x23` alike.
* **What `flag` and `B` mean in effect `0x23`** (§6.2), and what the `+0x50` vtable predicate
  at `14278e22d` is.
* **What the `0x02F2` key indexes** (§6.3) — `0x143AA8328` was not identified, only the `spec`
  string it reaches.
* **What `nId` other than `0x13DA0E`/`0x13DA0F` does.** Nothing, on this path — but I did not
  check whether the id is used by anything else that reads the same node.
* **Whether an EXP indicator exists** anywhere. §3 retracts one function's naming and claims
  nothing beyond it.
* **The name of the object at `0x143AC92C8`.** Its behaviour is measured; its name is [I].
* **The second `0x007C` trailer's charm value** (`142d548b6` → `FUN_1428a7f00`). Untouched.

---

## 10. Reproducing every number in this document

Repo root as the working directory, always — `CLAUDE.md` § "The scratchpad shadows the real
tools".

```
python tools/reads.py    0x140304100 2                 # the control: mixed direct + helper
python tools/listing.py  0x140304100 | grep READ       # must agree with it, address for address
python tools/callers.py  0x1402fa9a0                   # the control: 96 sites, 43 in 0x140304b20
python tools/dataref.py  0x143a46f48                   # the control: 2 readers

python tools/listing.py  0x142d54780 0x1402cbb50 0x140fd31f0   > research/msexe-statchange-handler.txt
python tools/listing.py  0x140fd31f0                            > research/msexe-recovery-popup.txt
python tools/listing.py  0x142771360                            > research/msexe-shownumber.txt
python tools/listing.py  0x140ee8f00                            > research/msexe-numberset-loader.txt
python tools/listing.py  0x14281e2d0                            > research/msexe-numberqueue-push.txt
python tools/listing.py  0x14281e390                            > research/msexe-recovery-draw-cand.txt
python tools/listing.py  0x1427863f0                            > research/msexe-usereffect-handler.txt
python tools/listing.py  0x14289a3a0 0x142771360                > research/msexe-localuser-dispatch.txt
python tools/listing.py  0x1428ee7d0                            > research/msexe-incdec-number.txt

python tools/callers.py  0x14281e2d0      # ONE site: 0x142790848 in FUN_1427863f0
python tools/callers.py  0x1428ee7d0      # ONE site: 0x14289ad10, i.e. opcode 0x02F2
python tools/callers.py  0x142771360      # 30 sites in 23 functions
python tools/callers.py  0x140e0ccb0      # 11 sites in 10 functions
python tools/reads.py    0x1428ee7d0 3    # ONE read in the whole subtree: 0x1428ee7f0 u32
python tools/dataref.py  0x143AC92C8      # 139 refs; the three in FUN_142d54780 are here
python tools/dump_va.py  0x140ee97cc 32   # the digit-set jump table
python tools/dump_va.py  0x1432b0b00 768  # NoRed0..NoCri3 as UTF-16
```

**Do not read `callers.py`'s `first`/`last` columns as the site list.** Four of the 23 caller
functions of `FUN_142771360` call it more than once — `0x1427863f0` (3), `0x1427a5a20` (3),
`0x1428923e0` (3), `0x1428ee7d0` (2) — and `first`/`last` cannot show a middle site. That is
what §5's warning box is about. Walk each caller's listing for every `call <target>` and
assert the count comes back to the total `callers.py` printed.

The two jump-table resolutions (`0x142791348` for the effect type, `0x14289d660` for the
local opcode) were done with a throwaway script that reads a `dword` table through
`tools/dump_va.py`'s section loader and reports which key's block contains a given address —
the section deltas differ between `.text` and `.rdata`, and a single hard-coded delta gives a
plausible, wrong answer for one of them. If it is rebuilt, check it against `0x14289a439 →
0x2d1` first, which `net::stats::USER_EFFECT_LOCAL` already asserts independently.

The 0x39b8 producer (§5) came from a one-off superset of `tools/rangescan.py` that matches on
the rendered operand text rather than on memory-operand displacements, so it sees immediates.
That difference is the entire finding; if this needs redoing, `rangescan` is the wrong tool
and widening its range will not help.
