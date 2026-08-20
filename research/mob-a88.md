# `mob+0xa88` is not the animation object, and the byte that kills the hit box is offset 91

Written 2026-08-20, in answer to two questions: *what is `mob+0xa88`* and *what makes
`FUN_141cd1620` run*. No client run, no Ghidra (another agent holds the lock) - capstone
through `tools/*.py`, the WZ through `target/release/wz-dump.exe`, and the fixture
`research/fixtures/mob-a88-null-on-every-mob-{world,hook,exit}.log`.

Markers: **[L]** read off a listing, a capture or the WZ; **[D]** derived from two or more
[L]; **[I]** inferred, including anything from the v214 reference.

---

## 0. Answer up front

| | |
|---|---|
| **`mob+0xa88` is the mob's AVATAR LOOK renderer, not its animation object** | it is allocated only from `FUN_141cd1620`, which builds a ~0x1e2-byte avatar-look structure and applies it. The label "the animation object" is wrong. The animation object is **`mob+0x610`** (with `mob+0x618`), and `encodeInit` creates it at `141c514ec`. §1 |
| **and `0` is its correct value for every mob in this client** | `FUN_141cd1620` allocates it only when `template+0x1e18` is non-null, and `template+0x1e18` is parsed from the WZ node **`avatarLook`**. **0 of 193 mob images have `avatarLook`**; the control `bodyAttack` resolves in 193 of 193. §2 |
| **what makes `FUN_141cd1620` run** | **exactly one caller in the whole image: `encodeInit` itself, `FUN_141c4ff80` at `141c53a40`.** Every jump that skips that call site lands on a fatal `E_POINTER`/`int3` tail, so on any non-fatal path through `encodeInit` it runs. It then bails without allocating, because of the WZ. §2 |
| **nothing our `0x03C6` sends can change that, and nothing should** | the only two inputs to the allocation are the caller's look pointer (`[rsp+0x80]`, which `141c53392` has just released to NULL) and `template+0x1e18` (pure WZ). No byte of the packet reaches either. §2.3 |
| **two gate readings in `mob-gates-arm-c.md` §5 are wrong, and I am saying so rather than editing their file** | `141cb4645` is **not** "NULL -> RETURN, WRITING NOTHING" - it is a branch into an 8 KB alternative that writes the body rect at `141cb6358`. `141c57185` is **not** a bail - `141c5721b` is the ordinary non-avatar path, which emits a real positioned rect and returns 1. §3 |
| **the field that is actually wrong is one we send: body offset 91** | `141c507bb` reads a `u32` into **`mob+0xd64`**, which is the mob's **size percentage**. The constructor sets it to **100** (`141c4de3c mov r13d,0x64` -> `141c4e17b`). **`crates/net/src/mob.rs:398` sends `0`.** §4 |
| **and 0 collapses the body rect to nothing, silently** | `FUN_141c57120` inflates the rect by `(scale - 100)% of half its width` unless the scale is **exactly 100**. At 0 that is a deflate by exactly half the width. Worked against template 2's real WZ rects, **every stance ends `left >= right` or `top >= bottom`** - which `141d326ae`/`141d326ba` **skip**, leaving `r14b == 0` and the mob rejected at `141d327c6`. §5 |
| **and that is why it is invisible on screen** | the render path treats the scale as `<= 0 -> do nothing` (`141c530d4 test eax,eax / jle`, `141cb623d cmp .,0 / jle`). The hit-box path treats it as `!= 100 -> scale`. **Same field, two different zero conventions.** The snails draw at natural size and have no hit box. §5.3 |
| **`appear_type` is not involved** | the fix is body offset 91, a `u32`, in `encodeInit`. It does not touch `appear_type` (offset 41) and must not: `>= 0` there writes `1` into `mob+0x504`, which gate 2 rejects on. §6 |

---

## 1. What `mob+0xa88` actually is

### 1.1 The two writers, re-derived

`tools/fieldrefs.py` control reproduced first (`+0x2f4 --write`, mob range -> exactly
`141c4d261`, `141c4e6ee`, `141cb7ef3`). Then, **without** `--write`, so `lea` handoffs are
visible: [L]

```
python tools/fieldrefs.py 0xa88 --lo 0x141c40000 --hi 0x141d60000
  53 hit(s), 445 resync point(s)
```

Of the 53, two are stores: `141c4ddbb` (the constructor's null) and `141cd1950` in
`FUN_141cd1620`. That reproduces `mob-gates-arm-c.md` §5.5's count. **There is also a third
way it is written and neither of us saw it with a write scan**: `140d2d420(&mob+0xa80)` ends
`mov qword ptr [rdi+8], 0`, and `mob+0xa80 + 8` is `mob+0xa88`. It is the release half of a
ref-counted pointer pair. `FUN_141cd1620` calls it at `141cd1944` immediately before
re-allocating. [L]

### 1.2 What gets stored there

```asm
141cd1931  mov  rbx, qword ptr [r14 + 0xa88]
141cd1938  test rbx, rbx
141cd193b  jne  0x141cd1972              ; already have one -> reuse
141cd193d  lea  rcx, [r14 + 0xa80]
141cd1944  call 0x140d2d420              ; release the old one (writes mob+0xa88 = 0)
141cd1949  xor  ecx, ecx
141cd194b  call 0x140d2d280              ; allocate
141cd1950  mov  qword ptr [r14 + 0xa88], rax
```

`FUN_140d2d280` is a **pooled, ref-counted allocator**: a spinlock on a singleton
(`[rip -> 0x143AC8688]`, `gs:[0x30]` as the owner tag), a free list at `pool+0x28`, and on a
miss `0x14019d3c0(0xfd8, 0x10)`. It then `memset`s `obj+0x28` for **0xfb0** bytes, installs
two vptrs (`obj+0x00 = 0x1436cc6e8`, `obj+0x20 = 0x1436cc6f0`), calls **`FUN_140f7dcf0(obj+0x28)`**
as the payload constructor, sets the refcount at `obj+8` to 1, and returns **`obj + 0x28`**.
So `mob+0xa88` points at a **0xfb0-byte payload with a 0x28-byte ref-counted header**, and
`140d2d420`'s `[ptr-0x20]` refcount / `ptr-0x28` object base agree with that to the byte. [L]

### 1.3 Its methods are the `0x140f8` avatar band, and they are shared with the UI

`tools/callers.py 0x140f82820` - the function `FUN_141cb4600` calls with `[mob+0xa88]` in
`rcx` - gives **16 call sites in 14 functions**, spread across `0x1417f`, `0x1420a`,
`0x1420b`, `0x14276`, `0x14279`, `0x1427b`, `0x1427e`, `0x14284`, `0x14285`. Two of them are
the mob's (`141c571b1`, `141cb4654`). It is a **shared class used by character/UI code**, not
a mob-private one. [L]

### 1.4 The decisive evidence: `FUN_141cd1620` builds an *avatar look*

`FUN_141cd1620(mob, lookRef)` builds a local at `[rbp+0x110]`, vptr `0x143279968`, zeroed
from `+0x110` to `+0x2f2` - about **0x1e2 bytes**. It fills it one of two ways: [L]

* `141cd176a call 0x1402eeb50(&local, [lookRef+8])` - copy a look the caller supplied;
* `141cd192c call 0x1402ecf60(&local, [node+4], [node+8], [node+0xc], [node+0x10], node)` -
  build one from the WZ node fetched via the template.

`0x1402eeb50` and `0x1402ecf60` sit in the same tight band as **`FUN_1402ee8d0`**, which this
repo already decompiled twice as the **avatar look** reader
(`research/msexe-avatarlook.c`, `research/avatar-look-reader.c`;
`research/charrecord-reuse.md` §3 calls it "the avatar look - the full body", and
`research/equip-block.md` uses it to dress the character list). `FUN_1402ee8d0` writes up to
`param_1 + 0x1bd`, the same ~0x1e2-byte struct. `research/msexe-itemalloc.c:946` calls
`FUN_1402eeb50` on an equip. [D]

> **So the label is wrong.** `mob+0xa88` is the object that draws a mob **as a player
> character** - hair, face, equipment - the way a Mimic or a disguised event mob renders. It
> is not "the animation object", and a mob without one renders and animates perfectly well.

### 1.5 The animation object is `mob+0x610`

`fieldrefs.py 0x610 --write`, mob range: **two** writers - `141c4d92a` (the constructor's
null) and **`141c514ec` inside `encodeInit` itself**. [L] Everything that behaves like an
animation object goes through it:

| where | what |
|---|---|
| `141cb4685 mov rbx,[r15+0x610]` | the **non-avatar** branch of `FUN_141cb4600` walks frames through it, and returns immediately if it is null (`141cb468f`) |
| `141c530be`, `141c5310b` | `encodeInit` pushes the scale into it |
| `141ce174e` | the runtime scale command pushes the scale into it |
| `141cd1982` | `FUN_141cd1620` reads it |

The fixture measured `mob+0x42c` alternating between `-18` and `-17` across 40 readings -
i.e. the frame changing under a live walk cycle - which is only possible if `mob+0x610`
exists and is running. **The animation object is present and working; the avatar object is
absent, as it should be.** That resolves the contradiction the task names. [D]

---

## 2. What makes `FUN_141cd1620` run

### 2.1 One caller, and it is `encodeInit`

```
python tools/callers.py 0x141cd1620
  0x141cd1620: 1 call site(s) in 1 function(s)
      0x141c4ff80      1 call site(s)   first 0x141c53a40  last 0x141c53a40
  0 tail jmp site(s).  0 qword pointer(s).
```

Control reproduced immediately before (`0x1402fa9a0` -> 96 sites / 15 functions, 43 in
`0x140304b20`). `0x141c4ff80` is the `[mobvtbl+0x38]` `encodeInit` that decodes our `0x03C6`
body. [L]

### 2.2 The call site is on the only non-fatal path

Every jump inside `encodeInit` whose source is before `141c53a40` and whose target is after
it: **67 jumps, 59 distinct targets, minimum target `0x141c53dcb`.** `141c53dcb` is
`mov ecx,0x80004003 (E_POINTER) / call 0x142ef3ac0 / int3`. Every one of the 59 is in that
fatal tail. **Nothing skips `141c53a40` and continues.** [D]

Since the mobs render, move and produce 990 `0x02FF` reports, `encodeInit` returned normally,
so `FUN_141cd1620` **ran** on every mob in the capture. [D]

### 2.3 It ran, and bailed - on the WZ, not on the packet

```asm
141cd175a  mov  rdx, qword ptr [rdx + 8]     ; the caller's look
141cd1761  je   0x141cd1774                  ; null -> go to the template
141cd1774  mov  rcx, qword ptr [rcx + 0x3a8] ; the mob TEMPLATE
141cd177e  jne  0x141cd179c                  ; null template -> BAIL (141cd20e3)
141cd17a0  call 0x14049c8a0                  ; out[8] = template[0x1e18], addref
141cd17e2  mov  rsi, qword ptr [rsi + 8]
141cd1865  jne  0x141cd18d8                  ; NULL -> BAIL, no allocation
...
141cd1931  (the allocation of mob+0xa88)
```

Both inputs are out of our reach: [L]

* **the caller's look** is `[rsp+0x78]` in `encodeInit`, and `141c53392 lea rcx,[rsp+0x78] /
  call 0x141d232a0` is a **release** whose last instruction is `mov qword ptr [rdi+8], 0`.
  So `[rsp+0x80]` is NULL at `141c53a38`, guaranteed, one branch earlier.
* **`template+0x1e18`** has exactly two qword writers in the whole image, and the sweep that
  says so really did cover `.text` and `.boot` (**19 hits, 666 939 resync points**):
  `1404a61e8` in `FUN_1404a5e10`, the template initialiser, storing the `1404a5e27 xor esi,esi`
  zero alongside `+0x1dd8`/`+0x1de0`/`+0x1de8`/`+0x1e20`; and `140494933` in `FUN_1404948b0`,
  the parse. Each of those two functions has **one** caller and it is the mob template parser
  `FUN_14047d990` (`14047dad1` and `14047fda9`). The three remaining non-read rows are
  `lea [rdx+0x1e18]` in the `0x142f`/`0x1431` bands, outside any template code - a different
  class at the same displacement.

### 2.4 `template+0x1e18` is the WZ node `avatarLook`

At `14047fd14` the parser builds a **10-byte narrow string** into a `std::string` and asks
the mob image root for that child:

```asm
14047fd14  movsd  xmm0, qword ptr [rip -> 0x14328ad10]   ; 8 bytes
14047fd20  movzx  ecx,  word  ptr [rip -> 0x14328ad18]   ; + 2 = 10
14047fd2b  mov    edx, 0xa                               ; length 10
14047fd69  call   0x14090f030                            ; GetItem(root, name)
14047fda9  call   0x1404948b0                            ; parse it -> template+0x1e18
```

`0x14328ad10` is `61 76 61 74 61 72 4c 6f 6f 6b` = **`avatarLook`**. [L]
(Its neighbours confirm the read: `0x14328ad00` is `offsetZ`, which the parser stores to
`template+0x2d4` twelve instructions earlier.)

`FUN_1404948b0` allocates nothing if the node has no children
(`140494915 cmp [rbp+0x67], r14d / jbe 140494da7`), so an empty `avatarLook` is the same as
none. [L]

### 2.5 No mob in this client has one

Every one of the **193** images in `client-patched/Data/Mob/Mob_000.wz` dumped with
`wz-dump cat` and searched:

| key | images carrying it |
|---|---|
| `bodyAttack` (positive control) | **193** |
| **`avatarLook`** | **0** |

The control resolves in every file, so the zero is a real zero and not a broken search -
the same method and the same control `research/mob-spawn.md` §11.2 used for `patrol` and
`targetFromSvr`. [L]

> **Conclusion for task two: nothing we send can reach the allocation, and nothing should.**
> `mob+0xa88 == 0` on all 40 readings is the *correct* value for a snail, and would be the
> correct value for every mob in the game. It is not the bug. There is no packet to add and
> no field to change.

---

## 3. Two readings in `mob-gates-arm-c.md` §5 that do not survive

Recorded here rather than edited into their file, as instructed.

### 3.1 `141cb4645` is a branch, not a return

`mob-gates-arm-c.md` §5.5 has:

```
FUN_141cb4600   141cb4645  je  0x141cb4680          ; <<< NULL -> RETURN, WRITING NOTHING
```

`0x141cb4680` is not the epilogue. `FUN_141cb4600` is **8814 bytes** and `141cb4680` opens
the **alternative branch**, which needs `[r15+0x610]`, `[r15+0x2c8]` and `[r15+0x2d0]`, walks
the animation frames, and ends at: [L]

```asm
141cb634d  lea  rdx, [r13 + 0x28]
141cb6351  lea  rcx, [r15 + 0x42c]
141cb6358  call 0x14080f550            ; <<< the body rect IS written here
141cb635d  mov  eax,[r15+0x434] / neg / mov [r15+0x43c],eax   ; and mirrored into 0x43c
141cb636d  mov  eax,[r15+0x430]        / mov [r15+0x440],eax
141cb637b  mov  eax,[r15+0x42c] / neg / mov [r15+0x444],eax
141cb638b  mov  eax,[r15+0x438]        / mov [r15+0x448],eax
```

So `mob+0x42c` has **two** setters, one per branch, and the one for an ordinary mob is
`141cb6358`. §5.3 of that file quotes its own `fieldrefs` output as "three such handoffs";
the scan actually returns **four**, and `141cb6351` is the fourth. Re-running it here gives
10 rows including `141cb6351` and `141cb637b`. [L]

That the plain path is the live one is **measured, not argued**: the fixture's
`[rcx+0x42c] = 0xffffffee / 0xffffffef` are `-18` and `-17`, and template 2's WZ frames are
`move/0 lt.x = -18`, `move/1 lt.x = -17`, `move/2 = -17`, `move/3 = -17`, `move/4 = -18`,
`stand/0 = -18`. Two instruments, one number. [D]

### 3.2 `141c57185` is a fork, not a bail

```
FUN_141c57120   141c57185        mob+0xa88 == 0     -> bail 141c5721b
```

`0x141c5721b` is the **ordinary non-avatar path**, and it is the longer of the two: [L]

```asm
141c5721b  test   r8d, r8d                      ; r8d = the stance
141c57228  mov    eax, 0x43c
141c57232  mov    r15d, 0x42c
141c57238  cmove  r15d, eax                     ; stance 0 -> the mirrored rect
141c5723c  movups xmm0, xmmword ptr [r15 + rcx] ; <<< load the body rect
141c57241  movups xmmword ptr [rdi], xmm0       ; <<< into the OUTPUT rect
...        (the scale adjustment - section 5)
141c572a7  lea    rcx,[rsi+0x818] / call 0x1401b0340   ; + the mob's y
141c572b3  lea    rcx,[rsi+0x830] / call 0x1401b0340   ; + the mob's x
141c573c8  mov    eax, 1
141c573d6  ret                                   ; success
```

The all-zero write at `141c573d7` is reached only from `141c57154`, `141c5715f`,
`141c5716b` and `141c57178` - the degenerate-`0x42c` and `mob+0x98c` tests. **`mob+0xa88`
null does not produce a zero rect.** `mob+0x818`/`mob+0x830` are the obfuscated position
containers `research/mob-spawn.md` §6.4 pins as y and x, so this path emits a **world-space**
rect. [L]

The consequence for `mob-gates-arm-c.md` §0: "the real setter is gated on `mob+0xa88` ...
**Two gates, one field**" is wrong on both gates, and §5.4's revival of `move_action` rests
on the same chain. The animation state does still feed `[mobvtbl+0x68]` -> `FUN_141c57120`,
so §5.4's *structure* is right; what is not right is that `mob+0xa88` gates it.

---

## 4. The field that is wrong: body offset 91 -> `mob+0xd64`

### 4.1 The read and the store

```asm
141c507bb  call 0x1406e8c20            ; <<< READ u32   (body offset 91)
141c507c0  mov  dword ptr [rsi + 0xd64], eax
```

No conditional between them. `crates/net/src/mob.rs:398` emits `0u32` there and
`mob.rs:471` asserts it. [L]

That `141c507bb` **is** body offset 91 is not re-derived here; it rests on
`research/mob-spawn.md` §11.3, whose CFG dominator test picked out exactly the 35
unconditional reads in `encodeInit` and found them character-for-character the set `mob.rs`
emits, with the HP read as a positive control and the patrol read as a negative one. If that
mapping is off, the *offset* in the fix below moves but the *field* does not: it is whatever
byte position `141c507bb` consumes.

### 4.2 The constructor's default is 100

`fieldrefs.py 0xd64` over the mob range gives **26** uses and **two** writers inside a mob
class: `141c4e17b` (the constructor) and `141c507c0` (the packet). In the constructor: [L]

```asm
141c4de3c  mov   r13d, 0x64                        ; = 100
...        (two local branches, no other write to r13, no non-local entry)
141c4e17b  mov   dword ptr [rsi + 0xd64], r13d
```

`r13` is callee-saved, the last write to it before `141c4e17b` is `141c4de3c`, and the only
two branches in between (`141c4df63`, `141c4e01b`) are local. **A freshly constructed mob has
`mob+0xd64 = 100`, and `encodeInit` overwrites it with our zero.** [D]

### 4.3 It is a percentage, and there is a packet that sets it at runtime

`FUN_141ce1730(mob, packet)`: [L]

```asm
141ce173c  call 0x1406e8c20                  ; u32 from the packet
141ce1741  cmp  dword ptr [rbx + 0xd64], eax
141ce1747  je   0x141ce17c4                  ; unchanged -> nothing
141ce1755  mov  dword ptr [rbx + 0xd64], eax
141ce176b  call qword ptr [rax + 0x300]      ; on mob+0x610, arg 2
141ce17a4  call qword ptr [rax + 0x320]      ; on mob+0x610, arg 8, the new value
```

Its sole caller is `141d331a8`, inside `FUN_141d32b30`, the `0x03D9..0x044D` per-mob command
dispatcher. Its jump table is at `0x141d33448`, base `0x140000000`, index `opcode - 0x3d9`;
the entry `0x01d331a2` sits at `0x141d33554`, index **67**, so the opcode is
**`0x03D9 + 67 = 0x041C`**. A dedicated *set mob scale* packet exists and pushes the value
straight into the animation object. [D]

### 4.4 The v214 reference names it, for what that is worth

`Mob.java:2666` is `outPacket.encodeInt(getScale()); // 100?` and it sits immediately after
the `size`-then-`(int,int)` loop at 2660-2665 - our offset 87 count and loop, so its next
field is our offset 91. `Mob.java:192` is `this.scale = 100;`. **[I]**, a different game
version, and it agrees with the constructor default derived from the image. It also encodes
`getScale()` a *second* time at line 2641, which is our offset 46 - the one `mob.rs` already
sends as 100.

---

## 5. Why 0 destroys the hit box and leaves the sprite alone

### 5.1 The arithmetic

`FUN_141c57120`, after copying the local body rect into the output at `141c57241`: [L]

```asm
141c57244  mov  ecx, dword ptr [rcx + 0xd64]     ; scale
141c5724a  mov  eax, dword ptr [rsi + 0x434]     ; right
141c57250  sub  eax, dword ptr [rdx]             ; - left      = width
141c57252  cdq / sub eax,edx / sar eax,1         ; w2 = width/2, toward zero
141c5725b  lea  eax, [rcx - 0x64]                ; scale - 100
141c5726a  divsd xmm0, qword ptr [rip -> 0x143277e78]   ; = 100.0
141c57276  cvttsd2si r14d, xmm1                  ; r14 = w2 * (scale-100)/100
141c5727b  cmp  ecx, 0x64
141c5727e  je   0x141c572a7                      ; <<< only 100 EXACTLY skips this
141c57280  sub  dword ptr [rdi + 4], r14d        ; top    -= r14
141c5728a  add  ecx, r14d                        ; bottom += r14
141c5728d  sub  dword ptr [rdi], r14d            ; left   -= r14
141c57292  add  dword ptr [rdi + 8], r14d        ; right  += r14
141c57299  mov  eax, dword ptr [rsi + 0x438]     ; the local bottom
141c572a1  mov  dword ptr [rdi + 4], edx         ; top    = (top - bottom) + width + bottom
141c572a4  mov  dword ptr [rdi + 0xc], eax       ; bottom = the local bottom
```

`0x143277e78` reads `00 00 00 00 00 00 59 40` = **100.0**. [L]

With `scale = 0`, `r14 = -w2`, so the rect is **deflated by exactly half its width on each
side**, and the vertical branch reduces to `top' = top + width`, `bottom' = bottom`. [D]

### 5.2 Worked against template 2's real WZ rects

Template 2 is what the fixture spawned (40 of them, map 40). `wz-dump cat Mob_000.wz
0000002.img`: [L]

| stance | lt | rb | width | after the deflate | verdict |
|---|---|---|---|---|---|
| `move/0` | (-18,-26) | (19,0) | 37 | left 0, right 1, **top 10, bottom 0** | `141d326ba jge` **SKIP** |
| `move/1` | (-17,-25) | (19,0) | 36 | **left 1, right 1** | `141d326ae jge` **SKIP** |
| `move/2` | (-17,-25) | (19,0) | 36 | **left 1, right 1** | **SKIP** |
| `move/3` | (-17,-24) | (19,0) | 36 | **left 1, right 1** | **SKIP** |
| `move/4` | (-18,-24) | (19,0) | 37 | left 0, right 1, **top 12, bottom 0** | **SKIP** |
| `stand/0` | (-18,-26) | (19,0) | 37 | left 0, right 1, **top 10, bottom 0** | **SKIP** |
| `hit1/0` | (-21,-33) | (22,0) | 43 | left 0, right 1, **top 10, bottom 0** | **SKIP** |

Every stance the mob can be in fails one of the two per-rect filters. The general form is:
the horizontal test fails when the width is even, and the vertical test fails whenever
`width >= height` - which is true of every frame of this mob. [D]

With `scale = 100` the whole block is skipped at `141c5727e`, the rect stays
`(-18,-26,19,0)`, `141c572a7` adds the mob's position, and a 37x26 world box comes out.

That is the reported symptom exactly: **the vector is non-empty (filler 2 pushes
unconditionally at `141d32630`, so reject 26 at `141d32692` does not fire), every rect is
skipped at `141d326ae`/`141d326ba`, `r14b` stays 0, and `141d327c6` drops the mob** - with
all 17 documented gates green and 15 free output slots. 11 attacks, all 127 bytes.

### 5.3 The two zero conventions - why the snails still look right

| path | test | at `scale = 0` |
|---|---|---|
| **render**, `encodeInit` | `141c530d4 test eax,eax / jle 141c531df` | **skips** telling `mob+0x610` and `mob+0x618` about any scale |
| **render**, `FUN_141cb4600` | `141cb623d cmp dword [r15+0xd64],0 / jle 141cb634d` | **skips** the scaling and copies the raw canvas rect |
| **hit box**, `FUN_141c57120` | `141c5727b cmp ecx,0x64 / je` | **does not skip** - 0 is "scale to 0 %" |

`<= 0` means *"no scale set"* to the renderer and *"scale to nothing"* to the geometry. The
mob draws at its natural size, walks, and reports 990 movements - and has no hit box. **The
bug is invisible on screen by construction.** [D]

---

## 6. The change, and the warning it does not touch

> **`crates/net/src/mob.rs`, body offset 91, line 398: `0u32` -> `100u32`.**
> `141c507bb` -> `mob+0xd64`, the mob's size percentage, whose constructor default is 100.
> `mob.rs:471`'s assertion changes with it. Length is unchanged: 137 bytes, `u32` for `u32`.

**On `appear_type`, explicitly, because the task asks.** This answer **does not involve
`appear_type`** and the value at body offset 41 must not move. `appear_type >= 0` pulls in
the extra `u32` at `141c504d0` *and* writes `1` into `mob+0x504`, which gate 2 (`141d31ce5`)
rejects on - it would make every mob permanently unhittable, and it would look like a
different bug. Offset 91 is 50 bytes further into `encodeInit`, a different read
(`141c507bb` vs `141c50485`), a different width, and a different destination field
(`mob+0xd64` vs `mob+0x1168`/`mob+0x504`). The warning in `mob.rs` stays exactly as it is.

**What this does not claim.** That `scale = 0` collapses the rect is **[D]**, from the
listing plus the WZ, not from a watch. What is measured is `mob+0x42c = -18/-17` (which the
WZ reproduces to the number) and `mob+0xa88 = 0` (which the WZ explains). If the fix does not
work, the surviving candidate is `mob-gates-arm-c.md` §6 - **arm C never validates its own
attack rect** - which would look identical and is untouched by anything here. That is the
next thing to read, not `mob+0xa88`.

Testing the fix directly is cheaper than a watch: it is one byte on the server, the failure
mode is the current one, and a swing that damages a snail settles it.

---

## 7. Instrument notes

* **`tools/fieldrefs.py` control reproduced** (`+0x2f4 --write`, mob range -> exactly
  `141c4d261`, `141c4e6ee`, `141cb7ef3`) before every count above.
* **`tools/callers.py` control reproduced** (`0x1402fa9a0` -> 96 sites / 15 functions, 43 in
  `0x140304b20`, first `0x140304e49`) before every zero above. It is the 2026-08-20 version,
  which reports calls, tail jumps and data pointers separately; all three were zero for
  everything reported as single-caller.
* **`tools/listing.py` control reproduced** - `0x140304100` prints reads at `140304138 raw`,
  `140304144 u8`, `140304183 u8`, then a run of `u16`.
* **The WZ search had a positive control before it was believed** - `bodyAttack` in 193 of
  193 images, `avatarLook` in 0. A key that is present in a single file (`notAttack`,
  `noFlip`) resolves under the same method, per `research/mob-spawn.md` §11.2.
* **`tools/rtti.py --list` finds 1764 type descriptors and none of them are game classes**
  (`Avatar`: 0, `Look`: 0 - the list is lambdas and CRT types). The class identity in §1 is
  therefore [D] from behaviour and from this repo's own prior decompilation of
  `FUN_1402ee8d0`, not [L] from RTTI. Do not read `rtti.py`'s zero as evidence of absence.
* **A `[reg+disp]` write scan missed the release of `mob+0xa88` too.** `140d2d420` clears it
  through a `lea`'d `mob+0xa80`, exactly the shape `CLAUDE.md` warns about and exactly the
  shape that hid `141cb4647`. The habit that works is: run the scan **without** `--write` and
  read the `lea` rows.
* **Quoting a scan's output partially is the same failure as filtering it.**
  `mob-gates-arm-c.md` §5.3 printed three `lea` rows for `+0x42c` where the tool returns
  four, and the missing row is the setter for every ordinary mob. Paste the whole output or
  say how many rows there were.
* No client run. No Ghidra.
