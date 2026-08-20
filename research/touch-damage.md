# Touch damage: the client's hit packet is `0x00E5`, and it does **not** read the mob's body rect

2026-08-20. Static only - **no client run was spent on this.** Tags: **[L]** measured/read
off the image or the WZ, **[D]** derived from something [L], **[I]** inference or from the
reference source.

Listings added beside this file:

| | |
|---|---|
| `research/msexe-mobattack-141c69f40.txt` | `FUN_141c69f40`, the mob's per-frame attack resolver - the whole mob -> user damage machine |
| `research/msexe-mobupdate-141c626c0.txt` | `FUN_141c626c0` = `[mobvtbl+0x00]`, the mob update that calls it |
| `research/msexe-userhit-14288ac30.txt` | `FUN_14288ac30`, the central `0x00E5` builder |

---

## 0. The short answer

**The tidy hypothesis is falsified.** The client's mob -> player damage path is
`FUN_141c69f40`, it is reached once per mob update from `[mobvtbl+0x00]`, and **nothing in
it or in any of its nine per-attack-type handlers calls `FUN_141c57120`, `FUN_141c56e00`
(`[mobvtbl+0x10]`) or `FUN_141caafe0`.** The rectangle it tests against the player is the
**player's** body rect, and the rectangles it tests it *with* come from the mob's **attack
template**, not from the mob's body. So the null `mob+0xa88` does not explain the player
taking no damage. **[L]**

What it looks like instead: that machinery is driven entirely by entries taken from the mob
template's `attack<N>` nodes, and **a snail has none** - `Mob_000.wz/0000001.img` has
`info`, `move`, `stand`, `hit1`, `die1` and nothing else. **[L]** With no attack entries the
resolver walks two empty containers and returns.

Where that leaves the question: **I could not establish that this client has a body/touch
damage path at all.** `bodyAttack` is parsed into `template+0x7c` and the only reader I can
find for that field is an anti-cheat CRC. That is a negative with a named blind spot (§5.3),
not a proof - but it is the honest state, and it is *not* "the same root cause as the attack
direction".

---

## 1. The packet: `0x00E5`, and this retracts `mob-combat.md` §7.1 / §14

`research/mob-combat.md` §7.1 named `0x00E5` a candidate family and §14 eliminated it. **The
elimination was wrong**, and its reason inverts the actual evidence:

> "Fifteen builders on one opcode is a multiplexed channel, not a hit report."

In this game family **`USER_HIT` is a multiplexed channel** - one builder per damage source
(mob body, mob attack, obstacle, dot, reflect, field, ...), all writing the same struct. The
§14 note that `FUN_14185ad30` "calls the mob-pool lookup `FUN_141d2efc0`" was the correct
signal read the wrong way round: that is exactly what a hit-from-a-mob builder does.

### 1.1 What names it - a GM string, not a guess **[L]**

```
0x1434282b0  "/logUserHitDamage"
0x1434282c8  "Log user hit damage in chat window"
0x143484d60  "[Client] User Hit Damage: %d"
```

`tools/xref.py --string "[Client] User Hit Damage"` -> one reference, `0x142918b60`, inside
**`FUN_142918b40`** (87 bytes):

```asm
142918b46  cmp  dword ptr [rcx + 0x5070], 0   ; the /logUserHitDamage flag on the user
142918b4d  je   142918b91
142918b5d  mov  r8d, edx                      ; the damage
142918b60  lea  rdx, [rip -> "[Client] User Hit Damage: %d"]
142918b77  call 0x1415eca30                   ; print into the chat window, colour 0xb
```

`tools/callers.py 0x142918b40` -> **one** caller, `FUN_142771360` at `0x142771660`.
`tools/callers.py 0x142771360` -> **30 call sites in 23 functions**.

`research/msexe-send-opcodes.txt` lists **16 call sites in 14 distinct functions** for
`0x00E5` (§14 says "fifteen"; the extra is `FUN_1428923e0` appearing three times). Setting
the two lists against each other:

```
14 distinct 0x00E5 builders, 13 of which call FUN_142771360:
  14185ad30 141fd1260 14288ac30 14288cb40 14288d340 14288d4d0 14288d6d0
  14288de60 1428923e0 14289a3a0 1428aa0a0 142903600 142908220
the one that does not:  1429077f0
```

**13 of 14.** The only caller of the function that prints `[Client] User Hit Damage: %d` is
called by almost every builder of `0x00E5` and by nothing else that builds a packet. So
`FUN_142771360` is "the user just took `n` damage - show it and log it", and the functions
that call it are the functions that build `0x00E5`. **[L]**

Worked example, `FUN_14185ad30` (`research/msexe-packet-fields.txt` line for `0x00E5`):

```asm
14185ae7b  call 0x14025d6a0                    ; HITINFO ctor at [rsp+0x90]
14185ae80  mov  dword [rsp+0x94], 0xfffffff5   ; damageType = -11
14185ae99  call 0x1429e3ef0                    ; the update tick
14185aea5  mov  dword [rsp+0x98], edi          ; the damage
14185aeac  mov  edx, 0xe5
14185aeb1  lea  rcx, [rsp+0x140]
14185aeb9  call 0x1406ed520                    ; COutPacket(0x00E5)
14185aecf  call 0x14025d810                    ; serialise the HITINFO
14185aedc  call 0x1415d01c0                    ; send
14185aee1  neg  edi                            ; ... and the HP delta is negative
14185aef8  call 0x142771360                    ; show/log it
```

Its other arm resolves a mob and hands off to the central builder:

```asm
14185af10  mov  edx, [rcx+0x464]      ; a mob OBJECT id
14185af1d  call 0x141d2efc0           ; the mob-pool lookup
14185afe7  call 0x14288ac30           ; -> 0x00E5
```

### 1.2 The body **[L]**

Every `0x00E5` builder serialises one struct through `FUN_14025d810`
(`Encode(HITINFO* rdi, COutPacket* rsi)`), `0x14025d810 .. 0x14025da6c`. Read straight off
the listing, in wire order, with the struct offset each field comes from:

```
u32 [0x00]   u32 [0x04]   u32 [0x08]   u32 [0x10]   u32 [0x14]   u32 [0x18]
u8  [0x1c]   u32 [0x20]   u8  [0x24]   u32 [0x0c]   raw4 [0x28..0x2b]
u32 [0x30]   u32 [0x2c]   u32 [0x34]   u32 [0x38]
u8  [0x3c]   u8  [0x3d]   u8  [0x3e]   u8  [0x44]   u8  [0x3f]   u8  [0x48]   u8  [0x49]
u64 [0x50]
u32 [0x58] [0x5c] [0x60] [0x64] [0x68] [0x6c] [0x70] [0x74] [0x78] [0x7c]
    [0x80] [0x84] [0x88] [0x8c] [0x90] [0x94]
u8  [0x98]   u32 [0x9c]   u8  [0xa0]   u32 [0xa4]   u32 [0xa8]
```

31 x u32 + 11 x u8 + 4 raw + 1 x u64 = **147 bytes of body**. The `u8`s are all
`cmp byte [rdi+n],0 / setne dl`, so they are bools on the wire.

Field meanings that are anchored rather than guessed, from `FUN_14288ac30`
(`14288c5f6 .. 14288c6dd`, with the mob in `[rbp-0x20]`):

| struct | filled with | |
|---|---|---|
| `+0x04` | argument 3 | `-11` in the field-damage builder; the mob arms pass an index/id **[D]** |
| `+0x14` | `FUN_1429e3ef0()` | the update tick **[D]** |
| `+0x34`, `+0x38` | `FUN_141c54eb0(mob)` | mob-derived ids **[L that they come from the mob]** |
| `+0x3c` | a sign bit | direction / `left` **[I]** |
| `+0x70`, `+0x74` | `FUN_141c56830(mob)`, `FUN_141c56cc0(mob,0)` | mob-derived **[L]** |
| `+0x78` | argument 2 | **[L]** |
| `+0x84` | `FUN_141c54e30(mob)` | mob-derived **[L]** |

That is enough to recognise the packet on the wire; it is **not** a full decode, and no
capture of one exists.

### 1.3 The two central builders

`FUN_14288ac30` (7945 bytes, **44 call sites in 41 functions**) and `FUN_1428aa0a0`
(**26 call sites in 23 functions**). Every per-attack-type handler in §2 reaches both -
eight directly, the ninth through `FUN_141c6f830`.

### 1.4 The reference source agrees, and it is a candidate only **[I]**

v214/v265 `InHeader.java`: `USER_AREA_DOT_ATTACK(271) ... USER_HIT(275),
USER_ATTACK_USER(276), USER_CHAT(277)`. In this client `"User Dot"` is **`0x00E3`** and chat
is **`0x00E7`** (both measured), leaving exactly three slots - `0x00E4`, `0x00E5`, `0x00E6` -
where the reference has five. `0x00E4` has **no builder at all** in
`research/msexe-send-opcodes.txt`. So `0x00E5 = USER_HIT` fits the ordering. Scored 1 of 8
historically; this is corroboration, not evidence.

Two further orderings from the same file *did* check out against things already established
here, which is worth recording: `MOB_MOVE` -> `0x02FF` **[L]**, `MOB_APPLY_CTRL` -> `0x0300`,
`MOB_DROP_PICK_UP_REQUEST` -> `0x0301` - and `STATUS.md` already has "`0x0301` is a MOB
picking up a drop". So the `0x0300..0x0316` block is the client's mob-control set, and
**none of it is a "the mob hurt the player" packet.** **[D]**

---

## 2. The mob -> user damage path, and what geometry it actually reads

### 2.1 The chain **[L]**

```
FUN_141c626c0            = [mobvtbl+0x00], the per-frame mob update (20 427 bytes)
  141c659b6  call FUN_141c69b10   (mob, tick)
  141c659c1  call FUN_141c69f40   (mob, tick)      <- the damage resolver, ONE caller
```

`tools/callers.py 0x141c69f40` -> **1 call site, 1 function**: `FUN_141c626c0`. No tail jmp,
no data pointer.

### 2.2 It reads the **player's** body rect, through the same vtable slot **[L]**

```asm
141c6b3bb  mov  r15, [rip+0x1e3d156]   ; = 0x143AA8518, the CUser singleton
141c6b3c2  mov  [rbp-0x68], r15
141c6b3c6  mov  r8d, 1
141c6b3cc  lea  rdx, [rbp+0x358]
141c6b3d6  call 0x142995700
```

and `FUN_142995700` is a seven-byte thunk:

```asm
142995700  mov  rax, [rcx]
142995703  jmp  qword ptr [rax + 0x10]
```

So it is **`[vtbl+0x10]` on the user object** - the same slot number `research/mob-gates-arm-c.md`
identified as the mob's body rect (`[mobvtbl+0x10] = FUN_141c56e00`). Independent
corroboration that slot `+0x10` on this class hierarchy is `GetBodyRect(RECT*, int)`; the
client's own `/userbodyrect` debug command reaches the same slot at `142835c6a`. **[L]**

`[rbp+0x358..0x364]` is therefore `left, top, right, bottom` of the **player**.

### 2.3 The attack-type dispatch, and nine handlers **[L]**

`FUN_141c69f40` walks `mob+0x9d8` (a list of the mob's active attacks) and switches on
`[attack+8]`:

| `[attack+8]` | handler | at |
|---|---|---|
| `0`, `0xa` | `FUN_141c6de40` | `141c6b938` |
| `3,4,8,0x12` (`bt 0x40118`) | `FUN_141c6d690` | `141c6b69a` |
| `7` | `FUN_141c6dbb0` | `141c6b6c3` |
| `6` | `FUN_141c6d1d0` | `141c6b6e4` |
| `0xb` | `FUN_141c6f4e0` | `141c6b70b` |
| `0xe`, `0xf` | `FUN_141c70a00` | `141c6b90b` |
| `0x10` | `FUN_141c70e00` | `141c6b8ce` |
| (earlier arm) | `FUN_141cc6000` | `141c6b4d6` |
| default | `FUN_141c6cec0` | `141c6b8e1` |

Every one of the nine is handed the **player's** rect (`lea r8/rax, [rbp+0x358]`, or the mob
and the rect for the two that take it differently), and every one of the nine reaches
`FUN_14288ac30` and `FUN_1428aa0a0` - i.e. sends `0x00E5`. Eight call them directly; the
type-`0xb` handler `FUN_141c6f4e0` calls them through `FUN_141c6f830`
(`141c6fb7e` / `141c6fc27`), which has `FUN_141c6f4e0` as its only caller.

### 2.4 The hit test itself is a rectangle intersection - against the ATTACK's rects **[L]**

`141c6bc50 .. 141c6bcdd`, walking a copied list of rects (`[node+0x00..0x0c]`) against the
player's rect:

```asm
141c6bc81  r8d = [rdi+0xc]  ; cmovle from [rbp+0x364]   -> min(bottom, playerBottom)
141c6bc94  edx = [rdi+8]    ; cmovle from [rbp+0x360]   -> min(right,  playerRight)
141c6bca4  r9d = [rdi+4]    ; cmovge from [rbp+0x35c]   -> max(top,    playerTop)
141c6bcb7  ecx = [rdi+0]    ; cmovge from [rbp+0x358]   -> max(left,   playerLeft)
141c6bcc6  cmp ecx, edx / jge  -> empty, next rect
141c6bcca  cmp r9d, r8d / jge  -> empty, next rect
141c6bccf  call 0x141cc2120 (mob)
```

and the send is at `141c6bda1` (`FUN_14288ac30`) / `141c6be22` (`FUN_1428aa0a0`).

**This is the same shape as the melee collector's per-rect filter** (`141d326ae`), which is
why the hypothesis was worth testing. It is not the same rectangle.

### 2.5 Where those rects come from - the attack template, not the body **[L]/[D]**

`mob+0x9d0` (the container `mob+0x9d8` is the head of) has these writers in
`0x141c40000..0x141d60000`: the constructor `FUN_141c4cee0`, the teardown `FUN_141c4ed50`,
`FUN_141c69f40` itself (two erases), and **`FUN_141c71920`, `FUN_141c8a380`,
`FUN_141c912b0`, `FUN_141cc89f0`, `FUN_141cc8a20`**. `tools/rangescan.py 0x9d0` /`0x9d8`,
9 and 3 sites.

`FUN_141c912b0` is the rect builder. It takes the mob in `r15` and an **attack template** in
`r14` and computes the rect from the attack template's own offsets at the mob's position,
mirrored by the stance:

```asm
141c91439  mov  rax, [r15]
141c9143f  call qword ptr [rax + 0x68]      ; [mobvtbl+0x68] - the stance
141c91445  edx = [r14+0x198] ; negated when facing the other way
141c91459  r9d = playerX-ish + edx          -> [rbp-0x60]
141c91464  r8d = ... + [r14+0x19c]          -> [rbp-0x5c]
```

and `FUN_141cc8a20` - which inserts into `mob+0x9d0` - starts by fetching the attack:

```asm
141cc8a4c  mov  edx, [rbp+0x7f]        ; the attack index
141cc8a4f  mov  rcx, [rcx + 0x3a8]     ; the mob TEMPLATE
141cc8a56  call 0x140499fa0            ; template->GetAttack(idx)
141cc8a5b  mov  rbx, rax
141cc8a61  jne  ...                    ; NULL -> write 0 and leave
```

So the whole machine is fed by `template->attack<N>`. **[L]**

### 2.6 The scan that falsifies the tidy hypothesis **[L]**

For each of `FUN_141c69f40` and its nine handlers, the full `tools/listing.py` output was
grepped for `0x141c57120`, `0x141c56e00`, `0x141caafe0`, `[reg+0x42c]`, `[reg+0xa88]` and
`call qword ptr [reg+0x10]`:

| | body-rect producer? |
|---|---|
| `FUN_141c69f40` | **no** |
| `FUN_141c6de40`, `FUN_141c6d690`, `FUN_141c6dbb0`, `FUN_141c6d1d0` | **no** |
| `FUN_141c70a00`, `FUN_141c70e00`, `FUN_141c6cec0` | **no** |
| `FUN_141cc6000` | **no**. Its four `[rax+0x10]` sites (`141cc6297`, `141cc62e6`, `141cc6335`, `141cc6384`, `141cc68c3`) pass **rcx only, no rdx** - a one-argument virtual, i.e. `Release`, not `GetBodyRect(RECT*,int)` |
| `FUN_141c6f4e0` | reads **`mob+0xa88`** at `141c6f767` - see below |

*Positive control for that grep:* the same grep over the same files finds the
`call qword ptr [rax+0x68]` stance calls (`141c6a590`, `141c6e8fe`, `141c6eb98`, `141c70c37`,
`141c71037`, `141c6db08`) and every `0x14288ac30`/`0x1428aa0a0` call site, so it is capable
of matching. The zero for the rect producers is a real zero **for these ten functions**.

The one `mob+0xa88` in the tree is in the type-`0xb` handler `FUN_141c6f4e0`, which is a
single arm of nine. And `FUN_141c69b10` - the function `CMob::Update` calls immediately
*before* the resolver - **does** call `[mobvtbl+0x10]`:

```asm
141c69bb6  call qword ptr [rax+0x10]        ; the mob's body rect into [rsp+0x70]
141c69bbe  call qword ptr [rip+0x15f9014]   ; IsRectEmpty
141c69bc6  setne bl                          ; -> centre vs corner arithmetic below
```

but it is gated on `mob+0xbb8 != 0`, drives an object full of `movss`/`xmm` float work
(`FUN_142e65430`, `FUN_142e65120`) and sends no packet. It reads as a particle/effect
emitter anchored to the body rect, not a damage test. **[D]**

---

## 3. The snail has no attack node **[L]**

`target/release/wz-dump.exe cat client-patched/Data/Mob/Mob_000.wz 0000001.img`:

```
top-level nodes: info, move, stand, hit1, die1
info: level 1, exp 1, maxHP 30, maxMP 30, speed -60,
      bodyAttack 1, PADamage 1, acc 33, eva 0, pushed 1,
      summonType 1, notAttack 1, noFlip 1, hideLevel 0
```

`0000002.img` (map 40's snail) is the same set of nodes, `bodyAttack 1`, `PADamage 3`, no
`notAttack`. **Neither has an `attack1`.**

`FUN_140499fa0(template, idx)` therefore has nothing to return, `FUN_141cc8a20` bails,
`mob+0x9d0` stays empty, and `FUN_141c69f40` walks two empty containers. **A snail cannot
hurt the player through §2's machinery, and that is true whatever `mob+0xa88` contains.**
**[D]**

---

## 4. What the server owes the client: nothing geometric - CONFIRMED **[L]**

The same dump answers question 4 directly. Every animation frame carries its own rectangle:

```json
"move": { "0": { "origin": {"x":20,"y":82},
                 "lt": {"x":-10,"y":-36}, "rb": {"x":11,"y":-14},
                 "head": {"x":-4,"y":-31}, "delay": 120 } }
```

`lt`/`rb` are the per-frame body rectangle and they are **in the client's own `Mob.wz`**. The
`0x03C6` body is 137 bytes with all 35 unconditional + 17 gated reads accounted for
(`research/mob-spawn.md` §11.3) and none of them is a rectangle. **The server does not owe
the client a hitbox, and adding one is not a thing the protocol can express.** The mob's rect
is produced at runtime by `FUN_141c57120` from `mob+0x42c` plus the current animation frame -
which is the *client's* copy of that WZ data, and is why a null animation object (`mob+0xa88`)
zeroes it.

---

## 5. So which is it

Stated plainly, as asked, and in the order the evidence supports:

### 5.1 NOT "the same root cause". **[L]**

The mob -> player damage path exists, is bounded, and was read end to end. It does not call
`FUN_141c57120`, `FUN_141c56e00` or `FUN_141caafe0`, and the rectangle it compares against
the player comes from the mob's attack template, not from the mob's body. A null `mob+0xa88`
cannot be the reason the player takes no damage **by that path**.

### 5.2 The nearest thing to a cause: there is nothing registered to hit with. **[D]**

Everything §2 resolves comes from `template->attack<N>` and the snail templates have no
`attack` node at all. If the client's touch damage is meant to ride this machinery, something
has to *register a body attack* into `mob+0x9d0`, and I did not find that something.

### 5.3 What I could **not** establish, and the blind spot in the negative

`bodyAttack` lands at **`template+0x7c`** **[L]**:

```asm
14047efea  lea  rdx, [rip -> u"bodyAttack"]   ; 0x14328abb0
14047f010  mov  edx, 0xffffffff               ; GetInt default -1
14047f01d  mov  dword ptr [r13 + 0x7c], eax
```

- same function, same base register as `targetFromSvr -> [R13+0x1a0]`
(`research/mob-spawn.md` §11.2), so the base is the template.

**I found no functional reader of `template+0x7c`.** The method:

* whole-image `tools/fieldrefs.py 0x7c` -> **1269** sites in **700** functions;
* whole-image `tools/fieldrefs.py 0x3a8` (the mob's template pointer) -> **1380** sites in
  **730** functions;
* the intersection is **15 functions**, and the only two that handle a mob template are
  `FUN_14047d990` (the parser, a **write**) and `FUN_140496e30`, which walks
  `+0x74, +0x7c, +0x81, +0x82, +0x1a0, +0x1a2, +0x2b8` through `FUN_1402af360/500/550` -
  an **anti-cheat template CRC** (the reference source has `MOB_CRC_DATA_RESULT`);
* the four candidates inside the mob code range - `FUN_141c5d1c0`, `FUN_141c8aed0`,
  `FUN_141c8d1b0`, `FUN_141cf8310` - were read by hand and are all
  `lea rcx,[r+0x74] / mov edx,[r+0x7c] / call FUN_1401ba9d0`, i.e. an **array base plus its
  count** on an unrelated structure;
* bounded and clean: `tools/rangescan.py 0x7c 0x141c40000 0x141d60000` gives **48** sites and
  **none** of them is in `FUN_141c69f40` or any of its nine handlers.

**The blind spot, named as `CLAUDE.md` requires.** A function *handed the template as an
argument* reads `[rcx+0x7c]` and never touches `+0x3a8`, so the intersection above cannot
prove absence - it can only say "no function that also navigates a mob's template pointer
reads this field". `FUN_140499fa0(template, idx)` is proof that such argument-passing
functions exist. So §5.3 is **[D]**, not [L], and the honest form is:

> **A body/touch-damage path in this client was not found.** It may be a fifth thing -
> a path that never touches the mob template because the flag is copied into the mob object
> at spawn, or one that lives outside every range scanned. What is established is that it is
> **not** the `FUN_141c57120` body rectangle, and therefore not the same root cause as the
> attack direction.

### 5.4 What is ruled out

| | |
|---|---|
| the same root cause (`mob+0xa88` -> zero rect -> no collision either way) | **ruled out** for the path that exists, §2.6 |
| a client-side gate our `0x03C6` fails to set | **not** one of the gates in §2 - `mob+0x9d0`/`mob+0x120` are filled from WZ attack data, and no spawn-packet field reaches them |
| the server owing a hitbox | **ruled out**, §4 |
| a different cause on the mob side | **the live hypothesis**, and it is upstream of everything in §2 |

---

## 6. Claims in other agents' files that this falsifies or corrects

I do not own these files; recording the corrections here as instructed.

1. **`research/mob-combat.md` §7.1 and §14 - `0x00E5` eliminated.** Falsified, §1. §14's
   stated reason ("fifteen builders is a multiplexed channel, not a hit report") is the
   argument *for* it.
2. **`research/mob-combat.md` §14 - "the mob attack-type name table at `0x1432b4510` ... the
   mob-side twin".** Those strings are the **user** side. Each is reached through its own
   `const char*` global; `0x143a49318 -> "Body"` has exactly one reader, `0x1428d27c5` inside
   `FUN_1428d07c0`, which is the client's own **`0x00E2` "User Body"** attack builder.
   Likewise `"Melee"` -> the `0x00DF` builders, `"Magic"` -> `0x00E1`, `"Dot"` -> `0x00E3`.
   `tools/dataref.py` on each global; the table base `0x1432b4510` has 0 references because
   the strings are addressed individually.
3. **`research/mob-combat.md` §14.2 - "the server can decide a touch itself".** Still the
   right fallback, and §4 now makes it the *only* option if §5.3 stays negative.
4. **Corroboration, not a correction:** `research/mob-gates-arm-c.md` §5.1's
   `[mobvtbl+0x10] = FUN_141c56e00` is confirmed from an unrelated direction - `+0x10` is
   `GetBodyRect` on the shared base class, reached on the *user* object by the thunk
   `FUN_142995700` and by the `/userbodyrect` debug command at `142835c6a`.
5. **`FUN_141c56e50` is dead.** `tools/callers.py` reports 0 calls, 0 tail jmps and 0 qword
   pointers, and `tools/xref.py --va` reports 0 - with `0x140304100`'s known `.rdata` slot
   `0x14327e530` reproduced as the data-pointer positive control. It is byte-identical to
   `FUN_141c56e00` except `mov byte [rsp+0x20], 1` vs `0`, i.e. the fifth argument to
   `FUN_141c57120`. Nothing reaches it in this build. **[D]** - an indirect call through a
   computed pointer would still be invisible.

---

## 7. Probe watches, if a run can be spared

Three slots. `1415db360:ret`, `141b2a280:rdx=0` and `140304100:hits=200` stay.

| | what it answers |
|---|---|
| `watch@141c69f40:args=5:hits=60` | **does the mob damage resolver run at all?** No lines = `CMob::Update` never reaches it and the question is upstream of everything in §2. Lines = it runs, and §3 says it then finds nothing to resolve |
| `watch@140499fa0:ret:hits=40` | `template->GetAttack(idx)`. A snail should return **0** every time. This is the control that separates "no attack data" from "attack data but no contact" - and it is cheap because it fires from the mob update, not per frame per rect |
| `watch@14288ac30:hits=20` | **did the client ever build a user-hit packet from a mob?** If this fires once, §2 works and the problem is only the geometry; if it never fires while the player stands in a snail, §5.2 holds |

**A warning on a tempting fourth.** `watch@1406ed520:rdx=0xe5` would catch *any* `0x00E5`
from any source and is the single most direct "did the client try to report a hit" test - but
`0x1406ed520` is the `COutPacket` constructor and runs for **every packet the client sends**.
Whether `:rdx=` filters before or after the log line is written has not been checked in
`crates/grap-stub`. **Do not arm it without reading that first**; if the filter is applied
after logging it will flood `maplecw-hook.log` and cost the run.

`FUN_141c69f40` is called once per mob per update, so with 30 mobs on the field `hits=60` is
about two seconds of wall clock. That is deliberate: the question is *whether* it runs, and a
bounded count answers it without drowning the log.
