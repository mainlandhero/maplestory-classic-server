# Why NPCs fade in, and why `0x044F` cannot stop it

Written 2026-08-21. **No Ghidra** — another agent held the project lock for the whole
session. Everything below comes from `tools/reads.py`, `tools/listing.py`,
`tools/fieldrefs.py`, `tools/rtti.py`, two throwaway capstone sweeps piped in on stdin, the
repo's own `wz-dump`, and the text already in `research/`.

Markers: **[L]** read out of the image or a file on disk, **[D]** derived from those,
**[I]** inferred / candidate.

---

## 0. Answer up front

| question | answer |
|---|---|
| Does `0x044F` carry an appear type, like the mob's `appearType`? | **No. There is no such field.** [D] |
| Then what is the unexplained `u32`? | There are two of them and they are now both named. Reads 5/6 are a per-NPC override of a **template animation-timing pair**, inert at `-1`; read 15 is the **`info/back` cover selector**; read 21 is a **duration in ms**, not an alpha. [L] |
| And the `raw[8]`? | The **NPC-clock deadline** — an 8-byte time value, sibling of `[npc+0x4a4]`/`[0x4a8]`/`[0x4ac]`, all four owned by `Etc/NpcNoticeBoard.img` / `npcTimeHourPos` code. [L] |
| Where does the fade come from? | **Not from CNpc.** Every alpha write in the whole CNpc class is a compile-time constant — either fully opaque or fully transparent. There is no ramp to aim a packet field at. [D] |
| Can the server fix it by changing this body? | **No.** See §6 for what a run can still settle, and §7 for two real bugs found on the way that should be fixed regardless. |

---

## 1. The read list, cross-checked before anything was built on it

`CLAUDE.md`: *"field order from the listing, field meaning from the decompiler, and
cross-check the packet-read count between them first."*

```
python tools/reads.py 0x140304100 2        # the control the tool's own docstring demands
python tools/reads.py 0x141e36b20 1        # 20 reads
python tools/listing.py 0x141e36b20        # 20 READ marks, the same 20 addresses
```

The control printed the mixed direct/`via helper` output the docstring requires, so the
instrument speaks. `reads.py`, `listing.py` and `research/npc-spawn.md` §4 then agree on
**20 reads by `FUN_141e36b20`**, at the same twenty addresses, in the same order. Nothing
below rests on a disputed count. **[L]**

### The 64-byte body, by byte offset

Offsets are into the `0x044F` **body** (after the opcode), and match the offsets
`crates/net/src/opcode.rs`'s own test already asserts (`[21]` for the byte it calls `!flip`).

| bytes | type | store / consumer | meaning | mark |
|---|---|---|---|---|
| 0..3 | u32 | pool hash key, `[npc+0x190]` | **objectId** | [L] |
| 4..7 | u32 | `FUN_141e77b70` → `Npc/%07d.img` | **templateId** | [L] |
| 8..9 | i16 | `[npc+0x3f0]` | **x** | [L] |
| 10..11 | i16 | `[npc+0x3f4]` | **cy** | [D] |
| 12..15 | u32 | `[npc+0x5a8]` | **anim-time override A** — §3.1 | [D] |
| 16..19 | u32 | `[npc+0x5ac]` | **anim-time override B** — §3.1 | [D] |
| 20 | u8 | `[npc+0x1ac] = (v != 0)` | a **bool** — §7.1 | [L] |
| 21 | u8 | `[npc+0x1a0]` | an **action / animation index** — §7.1 | [D] |
| 22..23 | i16 | field foothold map | **fh** | [D] |
| 24..25 | i16 | `[npc+0x174]` | **rx0** | [I] |
| 26..27 | i16 | `[npc+0x178]` | **rx1** | [I] |
| 28..29 | i16 | `[npc+0x17c]` | range-2 low | [I] |
| 30..31 | i16 | `[npc+0x180]` | range-2 high | [I] |
| 32 | u8 | `[npc+0x270]` | **isEnabled** — §2 | [L] |
| 33..36 | u32 | `FUN_141e51de0(npc,v)` → `[npc+0x4a0]` | **cover/back selector** — §3.2 | [D] |
| 37..40 | u32 | `[npc+0x4a4]` | NPC-clock **item id** | [D] |
| 41 | u8 | `[npc+0x4a8]` | NPC-clock flag | [L] |
| 42..45 | u32 | `[npc+0x4ac]` | NPC-clock **hour*100 + minute**, `-1` = live clock | [L] |
| 46..53 | **raw[8]** | `[npc+0x4b4]` | NPC-clock **deadline** — §3.3 | [D] |
| 54..57 | u32 | `[npc+0x520]` | **notice-board / MapleTV type**, `0` = template default | [L] |
| 58..61 | u32 | gated, `FUN_141e57510(npc,v)` | **a duration in ms**, *not* an alpha — §3.4 | [D] |
| 62..63 | u16 | string length | a name/script override | [L] |

**Nothing in that table changes an appearance mode.** Every field is either geometry, a
show/hide boolean, or one of two entirely separate features (the notice board and the NPC
clock) that happen to be encoded in the same packet.

---

## 2. `[npc+0x270]` is a hard on/off, and `research/npc-chatter.md` §8.2 must be un-retracted

The decoder branches on it directly, at `0x141e38a62`: **[L]**

```asm
141e38a62  cmp  dword [r12+0x270], 0
141e38a6b  je   141e38a81                 ; DISABLED
           call 141e3b110 (npc, 0)        ; ENABLED: build and show
           call 141e681f0 (npc)
141e38a81: mov  rcx,[r12+0x128]           ; DISABLED: hide the displayer's layers
           call 140f8abf0 (obj, 0)
           call 140f8aee0 (obj, 0)
```

and `FUN_140f8abf0` / `FUN_140f8aee0` are the alpha primitive:

```asm
140f8ac08  mov    esi, 0xffffff           ; ARGB with alpha 00 -> invisible
140f8ac0d  mov    eax, 0xffffffff         ; ARGB with alpha FF -> opaque
140f8ac12  test   edx, edx
140f8ac14  cmovne esi, eax
      ...  call   qword ptr [rax + 0x200] ; layer->put_color(ARGB)
```

So `layer vtable +0x200` is `put_color(ARGB)` and `[npc+0x270]` picks between the two
extremes. **Binary. No intermediate value is reachable from this field.** [L]

### The correction

`research/npc-chatter.md` §8.2 states: *"`[npc+0x270]` is not written anywhere in
`FUN_141e36b20`"*, and on that basis says read 14's offset "should not be quoted".

**It is written, at `0x141e36df4`.** [L]

```
141e36df4   41 89 84 24 70 02 00 00     mov dword ptr [r12 + 0x270], eax
```

That scan looked for the `disp32` form and its stated positive control was the
constructor's `mov dword [rsi+0x270], 1` at `0x141e360a6`. `rsi` as a base needs **no SIB
byte**; `r12` as a base **always needs one** (ModRM `84`, SIB `24`). The control could not
exercise the encoding that was actually there, so it passed while the target was invisible —
`CLAUDE.md`'s *"prove it discriminates"* failure in its purest form, and *"the retraction
can be the mistake"* on top of it. `tools/fieldrefs.py` finds it (it decodes rather than
pattern-matches), and it is the second of only two writers in the whole CNpc range.

`crates/net/src/opcode.rs`'s `ENABLED` label on byte 32 was right all along.

---

## 3. The four fields that were "unknown", now named

### 3.1 Reads 5 and 6 → `[npc+0x5a8]`, `[npc+0x5ac]` — an animation-clock override, inert at -1

`tools/fieldrefs.py 0x5a8 --lo 0x141e30000 --hi 0x141e80000` returns **three** hits and
`0x5ac` **two**: the CNpc constructor, the decoder, and one consumer. **[L]**

```
141e36259  mov qword ptr [rsi + 0x5a8], 0xffffffffffffffff   in the constructor (both dwords)
141e4a6c4  mov ebp,  dword ptr [rdi + 0x5a8]                 in FUN_141e4a5d0
141e4a6ce  mov eax,  dword ptr [rdi + 0x5ac]                 in FUN_141e4a5d0
```

`FUN_141e4a5d0` is called **unconditionally** by the decoder at `0x141e389f2`. It sets an
animation clock on the NPC's layer three ways, and each block is the same shape: **[L]**

```asm
141e4a6c4  mov  ebp, [rdi+0x5a8]
141e4a6cc  js   141e4a727                     ; <0 -> skip this block entirely
141e4a6ce  mov  eax, [rdi+0x5ac]
      ...  imul ecx, ebp, 0xbb8               ; A * 3000
           sub  ecx, eax                      ; - B
           lea  eax, [rcx + rcx*4]            ; * 5
           lea  edx, [rax*2 - 0x3fff8ad5]     ; * 2, minus ~2^30
           call qword ptr [rax + 0x198]       ; layer->(+0x198)(time)
```

The same routine's unconditional path passes the bare constant **`0xC0000000` = -2^30** to
the same vtable slot (`141e4a768`). So `+0x198` takes a **signed time** whose "already
running, nothing to play" value is about -2^30, and `[npc+0x5a8]`/`[0x5ac]` are a per-NPC
override of the template pair `[tmpl+0x29c]`/`[tmpl+0x2a0]` used in the block above it. **[D]**

The server already sends `-1`/`-1`, which is the constructor default and skips the block.
**This is not an appear type**, and setting it to `0`/`0` would only re-write the same layer
with the same epoch the unconditional block already wrote. **[D]**

### 3.2 Read 15 (bytes 33..36) → `FUN_141e51de0(npc, v)` → `[npc+0x4a0]`

`FUN_141e51de0`'s first act is `mov dword [r15+0x4a0], ebx` — it stores the packet value —
and the strings it reaches are `Npc/%u.img/info/cover` and `Npc/%u.img/info/back`. **[L]**
It is the NPC's cover/back artwork selector, the same family as the classic-shop counter.
Sent as `0`; irrelevant to appearance. **[D]**

### 3.3 Read 19, the `raw[8]` → `[npc+0x4b4]` — the NPC clock's deadline

The consumer is `FUN_141e3e220`, which is **CNpc vtable slot 0** (§4), i.e. the per-frame
update. Around `0x141e3f703`: **[L]**

```asm
141e3f731  cmp  dword [rdi+0x4b0], 0        ; a GetTickCount stamp
141e3f762  mov  edx, 0x1388                 ; 5000 ms
141e3f7ae  lea  rcx, [rdi+0x4b4]            ; the 8 bytes
141e3f7b5  call 1408fc990
141e3f7f6  lea  rcx, [rdi+0x4b4]
141e3f7fd  call 1408f6cc0                   ; -> [rbp+8] and [rbp+0xc]
141e3f814  imul eax, edx, 0x64              ; hour * 100
141e3f817  add  eax, r8d                    ;      + minute
141e3f824  mov  dword [rdi+0x4ac], eax
141e3f82d  call 141e54e20                   ; redraw
```

`FUN_141e54e20` is the drawing half, and it holds `npcTimePos`, `npcTimeHourPos`,
`npcTimeMinPos`, `npcItemNamePos`, `npcItemNameArea` and `Etc/NpcNoticeBoard.img/%d`. **[L]**

So the four fields `[0x4a4]`, `[0x4a8]`, `[0x4ac]`, `[0x4b4]` are **one feature**: the
countdown/clock NPC. `[0x4ac]` is `hour*100 + minute` with `-1` meaning "read the real
clock" (and `-1` is what the constructor and the server both use); the `raw[8]` is the
**deadline the clock counts toward**, in whatever 8-byte time the client's `1408fc990` /
`1408f6cc0` pair speaks. **[D]** — the pair was not decompiled, so the exact epoch is open;
that it is a time and that hour/minute come out of it is [L].

`crates/net/src/opcode.rs`'s guesses ("present item id / state / time") were in the right
neighbourhood. Nothing here touches appearance.

### 3.4 Read 21 (bytes 58..61) — the field labelled `ALPHA` is **not** an alpha

```asm
141e38c51  call 1406e8c20                 ; u32
141e38c56  cmp  dword [r12+0x294], 0
141e38c5f  jne  141e38c6f                 ; gated
141e38c61  test eax, eax
141e38c63  je   141e38c6f                 ; and non-zero only
141e38c6a  call 141e57510                 ; FUN_141e57510(npc, v)
```

and `FUN_141e57510` opens: **[L]**

```asm
141e57532  cmp  qword [rcx+0x128], 0 ; jne -> return
141e57540  cmp  qword [rcx+0x108], 0 ; jne -> return
141e5754e  call 1429e3ef0            ; = "mov eax,[global+0x60]; ret"  - a clock read
141e57553  lea  esi, [r14 + rax]     ; param + now
```

The value is **added to the current time**. It never reaches `put_color`, and `FUN_141e57510`
contains no `+0x200` call at all. The `// 19 u32 ALPHA` comment in
`crates/net/src/opcode.rs` is wrong: the packet is currently telling the client
*"…255 milliseconds from now"*. **[D]**

The knock-on is worth saying out loud. `STATUS.md` records the 2026-08-19 fix as *"the body
had two zeros that mattered — read 12 is `isEnabled` and read 19 is `alpha`"*. Both were
changed in one launch. `isEnabled` is measured and correct (§2); `alpha` is not an alpha,
so **that half of the pair was never actually measured** — it rode along. This is the same
shape as the `white` EXP byte contradiction already listed in `STATUS.md`.

---

## 4. The fade is not in CNpc — the negative, with its controls

### The positive control: the mob's `appearType`, found the same way

`mob+0x1168` is a known field of a known spawn packet in the same client, so it is the
control the brief asked for.

```
python tools/fieldrefs.py 0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write
    -> the exact three rows its docstring names (141c4d261, 141c4e6ee, 141cb7ef3)
python tools/fieldrefs.py 0x1168 --lo 0x141c40000 --hi 0x141d60000
    -> 141c4e4f0  mov dword [rsi+0x1168], 0xffffffff   (constructor default = -1)
       141c50490  mov dword [rsi+0x1168], ebx          (the packet read)
```

Two hits, **both writes, no reader** — which would read as "nothing consumes appearType" and
is false. The consumer is the *local copy*:

```asm
141c5048a  movsx ebx, al
141c5048d  mov   [rbp+0x70], ebx            ; <- fieldrefs cannot see this
141c51a72  mov   r15d, [rbp+0x70]
141c51a76  lea   eax, [r15+6]
141c51a81  cmp   eax, 5
141c51a84  ja    141c52a95                  ; appearType >= 0 -> default
141c51a8c  mov   ecx, [r14 + rax*4 + 0x1c54024]   ; 6-entry jump table
```

Quoting the blind spot, from `tools/fieldrefs.py`'s own docstring: *"`rsp`/`rbp`/`rip`-based
operands are dropped: those are stack frames and globals, not fields."* It bit here, exactly
as `CLAUDE.md` warns for the `lea`-and-hand-off case. **I worked around it by grepping the
`listing.py` output for `rbp + 0x70`, not by re-running the scan.**

Table at `0x141c54024`, six int32 image-relative entries: **[L]**

| appearType | case |
|---|---|
| -6 | `0x141c52772` |
| -5 | `0x141c5274e` |
| -4 | `0x141c523bf` |
| -3 | `0x141c5236b` |
| **-2** | `0x141c51d59` — gated on `template[0x2c8]`, calls **`FUN_141c56870(mob, 5)`** first |
| **-1** | `0x141c51a99` — the same block **without** that call |
| >= 0 | default, `0x141c52a95` |

So the mob's "fade" is `-2` selecting **animation action 5** before the ordinary setup, and
`-1` skipping it. `research/mob-behaviour.md` and `crates/net/src/npcchat.rs` both treat the
action index as an animation selector, and `0x0453`'s `nAction = -1` means "no animation
change" — so action 5 is a spawn animation, not an alpha. **[D]**

**There is no jump table, no local copy, and no equivalent switch anywhere in
`FUN_141e36b20`.** The NPC decoder's only enum-shaped dispatch is `[npc+0x520]`'s
1/2/3 notice-board fork.

### The negative: nothing in CNpc computes an alpha

A capstone sweep of `0x141e35ea0 .. 0x141e75800` (constructor to pool) for
`call qword ptr [reg + 0x200]` — `put_color`, established in §2 — returns **15 call sites**,
and for **every single one of them** the nearest preceding write to `edx` is a
`mov edx, imm32`, and that immediate is one of exactly two values: `0xFFFFFFFF` (opaque) or
`0xFFFFFF` (alpha 0). Fourteen are the opaque constant; the one exception is in the update
virtual and picks between the same two:

```asm
141e3e86f  call 14158f2f0
141e3e874  test eax, eax
141e3e876  mov  edx, 0xffffffff
141e3e87b  je   141e3e882
141e3e87d  mov  edx, 0xffffff
141e3e885  call qword ptr [rax+0x200]
```

The same sweep over the NPC pool `0x141e75800 .. 0x141e7a000` returns **0**. Over the mob's
`encodeInit` `0x141c4ff80 .. 0x141c54054` it returns **2**, both constants — so the sweep
speaks and it is not NPC-specific.

**Coverage check, so the range is not just an assumption.** `tools/rtti.py --list Npc` finds
`.?AVCNpc@@` at `0x143a9d5e0`; its COL does not resolve (the same RTTI limitation
`research/mob-spawn.md` §5 records for `.?AVCMob@@`), so I took the vtable from the
constructor instead — `141e35ed3 lea rax,[rip+0x15dd986]` → **`0x143413860`**, 72 slots.
Slot 0 is `FUN_141e3e220`, the update/draw. **Every slot but six lies inside the swept
range**; the six that do not are `0x141b096a0`, `0x141b0c010/020/090`, `0x140939780/790`
and `0x1409397a0`, and sweeping them finds exactly one `put_color`, `neg esi; sbb edx,edx`
— again only 0 or -1. **[L]**

A second, non-overlapping instrument agrees. Dumping every rip-relative string the whole
CNpc range reaches (69 of them, including `delay`, `effect`, `Canvas`, `Npc/%07d.img`,
`Etc/NpcNoticeBoard.img/%d`, `npcTimeHourPos`) turns up **no `alpha`, no `fade`, no
`blend`** — while the same technique finds `alphaShowDelay`, `alphaRemoveDelay`, `fadeIn`,
`fadeOut`, `fadeTime` and `objectAlpha` elsewhere in the image, so the words exist and the
walk can see them. **[L]**

### And it is not the artwork either

Map 1's two NPCs, dumped with the repo's own `wz-dump`: **[L]**

```
target\release\wz-dump.exe cat client-patched\Data\Npc\Npc_000.wz 0000001.img
target\release\wz-dump.exe cat client-patched\Data\Npc\Npc_000.wz 0000002.img
```

Heena's `stand` frames carry `delay`, `origin`, `z` and an `_outlink` and **no `a0`/`a1`**;
Sera's single `a0` hit is `speak: {"0": "a0"}`, a String.wz key, not an alpha. So the
Gr2D frame-alpha ramp — the one mechanism that could fade a layer without any CNpc code
computing it — **is not present in the art for the two NPCs on the map where this was
reported.** [L] I did not enumerate all 308 NPC images; that is the named limit of this one.

### What this rules out, and what it does not

**[D] Ruled out:** any field of `0x044F` controlling a fade; CNpc computing an alpha;
map 1's NPC art carrying a frame alpha ramp.

**[D] Also ruled out — the "the whole screen fades in on a map change" theory.**
`crates/world/src/session/field.rs::on_field_entered` builds the NPC replies **and then
appends the mobs to the same `Vec<Reply>`**, on the same `0x00DC` trigger, in the same
burst, NPCs first. Mobs with `appearType = -1` are confirmed on screen to appear instantly.
Two object kinds, one packet burst, one of them fades: the transition cannot be the cause.

**Not ruled out, and this is the honest limit:** an alpha ramp living inside the Gr2D layer
implementation itself, reached through a vtable slot other than `+0x200` — `+0x198` (the
time setter of §3.1) and `+0x190` are both called on NPC layers and neither was decompiled.
If the coordinator wants that closed, **the function to decompile is `FUN_141e4a5d0`**
(1014 bytes, `0x141e4a5d0..0x141e4a9c6`) **together with whatever `layer->vtable+0x198` is**
— it is the only unconditional, time-computing call on the NPC creation path, and it is
where an "animation starts now" versus "animation started an eon ago" difference would live.
I am naming it rather than guessing at it, per the brief.

---

## 5. WIRE IT LIKE THIS

**There is nothing to wire. No byte of `npc_enter_field` selects an appear or fade mode.**

The mob fix does not transfer because the mob's mechanism is a *jump table on a signed
appear type that selects a spawn animation*, and the NPC decoder has no such table, no such
field and no spawn animation to select. Every one of the 20 reads is now attributed to a
store or a consumer (§1), and none of them reaches an alpha, a timer that gates visibility,
or an animation-mode switch.

**Do not** change bytes 12..19, 33..61 hunting for one. Byte 32 (`isEnabled`) is the only
field in the body that can affect whether an NPC is seen, it is already `1`, and `0` is what
made every NPC invisible for days.

### Two things in `crates/net/src/opcode.rs` that should change anyway

Both are wrong labels rather than wrong bytes, and I have not touched the file — the
coordinator owns it.

1. **Byte 58..61 is not `ALPHA`.** It is a millisecond duration added to the current clock
   (§3.4). Sending `255` is harmless but meaningless; `0` is the value that skips the call
   entirely (`test eax,eax; je`). Recommend `0` **and** a corrected comment, but note that
   this makes it a behaviour change, so it should not ride along with anything else.
2. **Bytes 20 and 21 look swapped.** See §7.1 — this one is a real bug with a visible
   consequence and is worth its own fix.

---

## 6. The single client test

**It does not touch the 64-byte body**, and that is deliberate: §4 says the body cannot
carry the answer, so spending a launch on a body variant spends it on a question already
closed. What is *not* closed is whether the fade belongs to **NPC creation** or to **the
moment of field entry**, and one test separates them.

### What to change

In `crates/world/src/session/field.rs`, keep `on_field_entered` exactly as it is, and add
**one extra `0x044F`** on the existing tick, **3 seconds after field entry**:

* `template_id` = `1` (Heena — art already loaded, guaranteed to exist)
* `object_id` = something that cannot collide, e.g. `9001`
* `x` = Heena's `x + 180`, so it stands clearly apart from the real one
* every other field **byte-identical to what `npc_enter_field` already produces**

### What the owner does

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

Log in, enter map 1 (Mushroom Town — West Entrance), and **stand still and watch a patch of
empty ground to Heena's right for about five seconds.** A second Heena will appear there.
The only thing to report is **how** they arrive.

### What each outcome means

| on screen | meaning |
|---|---|
| **The extra Heena fades in**, the same way the map-entry NPCs do, three seconds after the map has finished loading and settled | The fade is a property of **creating a CNpc**, unconditional and client-side. §4 says no field of `0x044F` can reach it, and this closes the question: **the server cannot fix this**, and the remaining work is `FUN_141e4a5d0` / `layer +0x198` (§4) or nothing at all |
| **The extra Heena pops in instantly**, solid from their first frame | The fade is **not** a property of NPC creation — it is tied to the field-entry moment. That makes it a *timing* problem, which the server owns: `on_field_entered` fires on `0x00DC`, ~420 ms after `SetField`, and the next step is to move or repeat the send rather than to change the body. This is the outcome that reopens the problem as fixable |
| **Nothing appears at all** | The extra send is broken, not the body. The body is byte-identical to the two that already work, so suspect the object id (a collision takes the pool's "already present" path — `or [obj+0x38],1` and return, reading nothing further, `research/npc-spawn.md` §5) or the tick never firing. **`world.log` names every `0x044F` that went out**; if the third one is not in the log the server never sent it |
| **The two map-entry NPCs stop appearing** | Then the change touched more than it should have. This is the `isEnabled`/`alpha` failure mode and it is why the test is specified as *add a third packet, change no existing byte*: if that happens, the edit was not the edit described here |

One variant. One observable. Both informative outcomes are decisive, and the third and
fourth rows are self-diagnosing from `world.log` without another launch.

---

## 7. Two bugs found on the way, neither of them the fade

### 7.1 The facing bit is being written into the animation action index

```asm
141e36bdc  call 1406e8ae0                ; body byte 20, u8
141e36be1  mov  ecx, r15d
141e36be4  test al, al
141e36be6  setne cl
141e36be9  mov  [r12+0x1ac], ecx         ; a BOOL

141e36bf4  call 1406e8ae0                ; body byte 21, u8
141e36bf9  movzx eax, al
141e36bfc  mov  [r12+0x1a0], eax         ; a VALUE
      ...
141e36d87  mov  ebx, [r12+0x1a0]
141e36db7  mov  [rsp+0x30], ebx          ; passed as the action argument to
141e36dd0  call rsi                      ;   displayer->vtable[0x118](...)
141e36dd5  call 1409c6d00
141e36dda  mov  [r12+0x1a0], eax         ; and the resolved action is written back
```

`[npc+0x1ac]` has 8 references in the whole class: three writes (two in the constructor, one
in the decoder) and **five reads, every one of them a `cmp` against zero** — it is a boolean.
`[npc+0x1a0]` has 16 references and a proper setter with change detection
(`FUN_141e39b90`: `cmp edi,[rbx+0x1a0]; mov [rbx+0x1a0],edi`) and is what `0x0453`'s
`nAction` addresses — `crates/net/src/npcchat.rs` already documents `nAction = -1` as
*"show a line without changing the NPC's animation"*. So byte 21 is an **animation index**.
**[L]** for the stores and the argument passing, **[D]** for "index into the template's
animation list".

`crates/net/src/opcode.rs` currently sends `0` into byte 20 (commented `move`) and
`u8::from(npc.f == 0)` into byte 21 (commented `!flip`). `research/npc-spawn.md` read 7
put the facing on byte 20. They disagree, and the byte shapes back `npc-spawn.md`: a
facing flag is a bool, an action is a value.

The live consequence on map 1: **Heena has WZ `f = 1` so they are sent action `0`; Sera has no
`f` so they are sent action `1`.** Sera's animation nodes, in WZ order, are
`stand, move, blink, angry, smile, image, alert, hair`. Byte 21 is read `movzx`, i.e.
**unsigned**, so `-1` ("leave the animation alone") is not expressible there — `0` is.
**Recommended, as its own change: send `0` for byte 21 and the facing bit for byte 20.**
Worth one launch on its own; not worth combining with §6.

### 7.2 The `0x0451` "already present" branch is still unread

`research/npc-spawn.md` §3.1 describes `0x0451` for `flag != 0` **and the id unknown**. What
it does when `flag != 0` and the id is **known** is not written down anywhere, and that
matters: if it re-reads the 20-field body, a 5-byte `0x0451` over-reads and faults the
client. Nobody should send `0x0451` after `0x044F` as a "does the controller matter" probe
until that branch is read. Flagging it because it is the obvious next idea and it is not
currently safe.

---

## 8. Instruments, and what each one could and could not see

| instrument | control run | blind spot, quoted |
|---|---|---|
| `tools/reads.py` | `0x140304100 2` produced the mixed direct/helper list its docstring demands | ten primitives; a stale scratchpad copy has nine — run from the repo root, which is what was done |
| `tools/listing.py` | its READ marks matched `reads.py` at all 20 addresses | shares `reads.py`'s loader, so it *cannot* disagree about extents — which is a guarantee, not a check |
| `tools/fieldrefs.py` | `0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write` reproduced its three documented rows exactly | *"`rsp`/`rbp`/`rip`-based operands are dropped"* — **this bit in this session**: the mob's `appearType` consumer is `mov r15d,[rbp+0x70]` and is invisible to it. Worked around by grepping the listing, not by re-running the scan. `--write` was **not** used for the read hunts, per `CLAUDE.md`'s *"a `[reg+disp]` write-scan cannot see a store through a pointer that was `lea`'d and handed off"* |
| `put_color` sweep (throwaway, piped on stdin) | found 15 sites in CNpc, 2 in the mob decoder, 1 in the out-of-range stubs, 0 in the pool — so it is not returning zero by construction | cannot see an indirect call whose target register was loaded far earlier (`mov rsi,[rax+0x200]` … `call rsi`), which is exactly the form `FUN_141e36b20` uses for slot `+0x118` |
| string walk (throwaway) | 69 strings in the CNpc range including `delay`, `Canvas`, `Npc/%07d.img`; and it finds `fadeIn`/`alphaShowDelay` elsewhere in the image | only rip-relative references, one level of pointer indirection; runtime-built `BSTR`s are invisible, and `FUN_141e57510` reaches all of its property names that way |
| `tools/rtti.py` | listed `.?AVCNpc@@` | **0 locators** — the COL RVA does not resolve on this image, same as `.?AVCMob@@`. The vtable came from the constructor's `lea` instead |
| `wz-dump` | dumped both NPC images with their `delay`/`origin`/`_outlink` intact | only the two NPCs on map 1 were checked; 306 others were not |

Not used, deliberately: `tools/callers.py` (scans for `call` only — 27 909 functions would
report zero callers while reached by a tail `jmp` or a data pointer), `tools/encodes.py`
(misses fields written by a loop, still unfixed), and Ghidra (locked by another agent for
the whole session).
