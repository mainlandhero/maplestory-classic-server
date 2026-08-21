# `FUN_141e4a5d0` is not a fade. It is the z-order setter, and the mob does the same thing.

Written 2026-08-21, **with Ghidra** (lock held for the whole session, no other agent using it).
Continues and **corrects** `research/npc-appear.md`, which ends by naming
`FUN_141e4a5d0` + layer `vtable+0x198` as the one unexamined thing on the NPC creation path.
That is the blocker this pass was sent to remove. It is removed, and the answer is negative.

Markers: **[L]** read out of the image or a file on disk, **[D]** derived from those,
**[I]** inferred / candidate.

New artefacts written by this pass:

| file | what |
|---|---|
| `research/msexe-npc-clocksync.c` | Ghidra decompilation of `FUN_141e4a5d0`, its mob twin `FUN_141cb9fd0`, and `FUN_142b56090` |
| `research/msexe-npc-clocksync.txt` | the linear listing of `FUN_141e4a5d0` (`tools/listing.py`) |
| `research/msexe-layer-iface-calls.txt` | every layer-interface vtable slot CNpc and CMob call, IID-confirmed |
| `research/msexe-putcolor-sweep.txt` | whole-image sweep of `call [reg+0x200]` and the edx that feeds it |

---

## 0. Answer up front

| question | answer |
|---|---|
| What does `FUN_141e4a5d0` compute? | `z = ((layer * 3000) + within) * 10 - 2^30 - k`, and pushes it to every layer the NPC owns. It is **`CNpc::UpdateZ`**. [D], with the arithmetic in §2 |
| What does `vtable+0x198` do with it? | It is the `put_` half of a `get_`/`put_` **int property pair** (`+0x190` reads, `+0x198` writes) on Gr2D layer interface `{6DC8C7CE-8E81-4420-B4F6-4B60B7D5FCDF}`. **[L]** for the pair and the interface; **[D]** for "that property is z" |
| Is it a fade or appear ramp? | **No.** `CMob` has a byte-for-byte twin, `FUN_141cb9fd0`, called unconditionally on every mob from the mob decoder at `0x141c52b80` — with the same helper, the same `A*3000 - B` formula, the same `-2^30` band and the same `0xC0000000` special case. **Mobs with `appearType = -1` are confirmed instant on screen.** A mechanism both classes share cannot be what makes one of them fade. **[L]** |
| Is the value reachable from the server? | **Yes — bytes 12..15 and 16..19 of `0x044F` reach this exact setter** (§3). But what they move is **draw order**, not appearance timing. There is no fade lever. |
| So where is the NPC fade? | **Not in CNpc.** Re-established here with a second, non-overlapping instrument (§4). The mob's own pre-fix fade was `Effect/Summon.img/%d`, a separate effect object, and the global that names it has **six readers, none of them in the CNpc range** (§5). **[L]** |
| Is there anything to wire? | **No.** §7. |

---

## 1. `FUN_141e4a5d0` — what it actually is

`0x141e4a5d0 .. 0x141e4a9c6`, 1014 bytes (`tools/pdata_lookup.py`). Called from five
functions plus one tail `jmp`; the one that matters is `0x141e389f2`, inside the `0x044F`
decoder `FUN_141e36b20`. **[L]**

It is one shape repeated. `rdi` is the NPC:

* `[npc+0x238]` — the NPC's main Gr2D layer (a COM interface pointer)
* `[npc+0x160]` — an interface on the NPC's own **displayer**; the concrete object is
  `[npc+0x160] - 0x20`
* `[npc+0x198]` — the NPC template
* `[npc+0x108]`, `[npc+0x128]`, `[npc+0x268]` — sub-displayers / sub-layers

The unconditional block, `0x141e4a610 .. 0x141e4a650`: **[L]**

```asm
141e4a5e8  mov  rax, [rcx+0x160]
141e4a5ff  lea  rsi, [rax-0x20]          ; the displayer, or 0
141e4a5f2  mov  r14, [rcx+0x238]         ; the layer
141e4a613  call 0x1409c6ce0              ; = secure_get(displayer + 0x320)   -> B
141e4a61d  call 0x1409c6cc0              ; = secure_get(displayer + 0x308)   -> A
141e4a622  imul ecx, eax, 0xbb8          ; A * 3000
141e4a628  sub  ecx, ebx                 ;   - B
141e4a62a  lea  eax, [rcx + rcx*4]       ;   * 5
141e4a630  lea  edx, [rax*2 - 0x3fff8ad5];   * 2, minus (2^30 - 29995)
141e4a63a  call qword ptr [rax + 0x198]  ; layer->put_?(edx)
```

`0x1409c6cc0` and `0x1409c6ce0` are two-instruction thunks — `add rcx, 0x308` /
`add rcx, 0x320` then `jmp 0x1401b0340` — and `0x1401b0340` is the client's **obfuscated
field reader** (a rolling XOR/rotate walk over a byte buffer). So A and B are two
ZtlSecure-protected ints on the displayer. **[L]**

The last block of the function is the tell:

```asm
141e4a957  call qword ptr [rax + 0x190]  ; [npc+0x238]->get_?(&t)
141e4a97c  call qword ptr [rax + 0x198]  ; [npc+0x268]->put_?(t)
```

and `[npc+0x108]` / `[npc+0x128]` get the same value handed to `FUN_140f81230(displayer, t)`,
which is `if (![disp+0xd60]) [disp+0xd58]->put_?(t)` followed by a walk over the displayer's
children. **The whole function reads one int off the main layer and stamps it onto every
other layer the NPC owns.** [L]

`+0x190` is a getter taking an `int*` out-param; `+0x198` is a setter taking an `int`. Both
return `HRESULT` (`test eax,eax; jns`), and both hand
`_com_issue_errorex(hr, iface, &DAT_14327fcb0)` the **same IID** on failure. **[L]**

---

## 2. The property is a z, not a time — and the constants prove it arithmetically

The GUID at `0x14327FCB0` is **`{6DC8C7CE-8E81-4420-B4F6-4B60B7D5FCDF}`**. It appears in
exactly two files on disk: `MapleStory.exe` (file offset `0x327ecb0`) and
**`Gr2D_DX11.dll`** (file offset `0x3d9468`, rva `0x3daa68`). In the DLL it sits inside an
ATL `_ATL_SIMPLEMAPENTRY` table at rva `0x44eb10` whose `dw` field is `0x10`, i.e. the
interface sub-object lives at offset `0x10` of the implementing class. **[L]**

I could **not** resolve the concrete vtable inside `Gr2D_DX11.dll` and therefore **cannot
name the method**. That is stated plainly here rather than guessed. What the constants say:

`-0x3FFF8AD5` is `-(2^30 - 29995)`, and `29995 = 3000*10 - 5`. So the expression is

```
z = ((A + 1) * 3000 - B) * 10 - 5 - 2^30
```

Now the whole family, every one of them a real call site found by the IID-confirmed sweep in
`research/msexe-layer-iface-calls.txt`: **[L]**

| site | constant | `2^30 - c` | as `30000 - k` | k |
|---|---|---|---|---|
| `141e4a630` **CNpc base, unconditional** | `0x3FFF8AD5` | 29995 | 30000 - 5 | **5** |
| `141e4a6a5` CNpc, template override | `0x3FFF8AD5` | 29995 | 30000 - 5 | 5 |
| `141e4a708` CNpc, **packet bytes 12..19** | `0x3FFF8AD5` | 29995 | 30000 - 5 | 5 |
| `141e4a816` CNpc, template-id whitelist | `0x3FFF8AD9` | 29991 | 30000 - 9 | 9 |
| `141cbaa80` CMob base helper, then `+1` at `141cba01e` | `0x3FFF8ADA` → `…AD9` | 29990 → 29991 | 30000 - 9 | **9** |
| `141cba68b` CMob switch case | `0x3FFF8AD9` | 29991 | 30000 - 9 | 9 |
| `141cba73d` CMob switch case | `0x3FFF8ADA` | 29990 | 30000 - 10 | 10 |
| `141cba33b` CMob switch case | `0x3FFF8ADB` | 29989 | 30000 - 11 | 11 |
| CMob `switchD_141cba15b_caseD_864703` (decompiler; the `lea` address was not read off the listing) | `0x3FFF8ADC` | 29988 | 30000 - 12 | 12 |
| `141cba4b6` CMob switch case | `0x3FFF8ADD` | 29987 | 30000 - 13 | 13 |

The `141e4a816` row is in `tools/listing.py`'s output but **not** in the IID-confirmed sweep:
its error path is a shared `jmp 141e4a77d` rather than an inline `lea r8,[rip -> IID]`, so
the 48-byte lookahead misses it. Named here so the file and the sweep are not read as
disagreeing.

Every constant is `-2^30 + 30000 - k` with `k` in **5..13**. The `*10` leaves exactly ten
slots per unit and the `k` ladder fills them: NPCs at slot 5, ordinary mobs at slot 9, boss
parts at 11/12/13. **That is a draw-order tie-break ladder between object classes, and it is
not a plausible shape for a time.** [D]

The second arithmetic check is stronger. One CNpc block, gated on a field flag, uses a
different constant: **[L]**

```asm
141e4a864  lea  edx, [rcx*2 - 0x3ffccbb0]     ; rcx = 5 * ([rax+4] + tmpl[0x90])
```

`2^30 - 0x3FFCCBB0 = 0x33450 = 210000`, and **`210000 = 7 * 3000 * 10` exactly** — the same
formula with `A = 7`, `B = 0`, no `-k`. A MapleStory map has exactly **eight object layers,
0..7**. So `A` is a layer index and `B` a within-layer depth, and this block means "put this
NPC at the top of layer 7". [D]

Third: the `0xC0000000` path — `-2^30` exactly, the floor of the band — is **not
unconditional** (§3) and fires only for a whitelist of template ids. In the mob twin the
identical `0xC0000000` path fires for template ids `0x7DBB93`, `0x7DBBAE`, `0x864702`,
`0x866E12` and a switch of ~20 more in the `0x8647xx` range — the Zakum/Horntail-shaped
multi-part background bosses. "Always draw at the very bottom" is exactly what those need.
**[L]** for the ids and the constant, **[D]** for the reading.

**Even if this property turned out to be something other than z, the conclusion in §0 does
not move**, because §3 shows the mob writes the same values through the same code and does
not fade.

---

## 3. Two corrections to `research/npc-appear.md` §3.1

**(a) The `0xC0000000` write is not unconditional.** `npc-appear.md` says *"The same
routine's unconditional path passes the bare constant `0xC0000000` = -2^30 to the same
vtable slot (`141e4a768`)"*. It is gated: **[L]**

```asm
141e4a727  mov  rax, [rdi+0x198]        ; the template
141e4a731  je   141e4a78b               ; no template -> skip
141e4a733  mov  eax, [rax]              ; template id
141e4a735  cmp  eax, 0x1781fa
           ... a six-way id compare ...
141e4a753  jne  141e4a78b               ; not on the list -> skip
141e4a768  mov  edx, 0xc0000000
141e4a770  call qword ptr [rax + 0x198]
```

The ids are `0x17815B` (1540443), `0x1781F8`..`0x1781FB` (1540600..1540603), `0x895902`
(8999170) and `0x895906` (8999174). No ordinary NPC takes this path. The genuinely
unconditional write is the **first** block, with the computed value. **[L]**

**(b) Bytes 12..19 do not write "the same epoch".** `npc-appear.md` says setting them
*"would only re-write the same layer with the same epoch the unconditional block already
wrote."* They would not. The packet block runs **after** the unconditional block and
overrides it: **[L]**

```asm
141e4a6c4  mov  ebp, [rdi+0x5a8]        ; packet bytes 12..15
141e4a6cc  js   141e4a727               ; < 0 -> skip (what the server sends today)
141e4a6ce  mov  eax, [rdi+0x5ac]        ; packet bytes 16..19
141e4a6d6  jns  141e4a6e0
141e4a6db  call 0x1409c6ce0             ; < 0 -> fall back to the displayer's B
141e4a708  lea  edx, [rdx*2 - 0x3fff8ad5]
141e4a70f  call r8                      ; layer->put_?(edx)
```

The two writes differ by `30000 * A_displayer`, which is zero only if the displayer's own
layer index happens to be 0. Confirmed by `tools/fieldrefs.py`: `[npc+0x5a8]` has exactly
three references in the whole CNpc range — the constructor default `-1` at `141e36259`, the
packet store at `141e36bac`, and this read at `141e4a6c4` — and `[npc+0x5ac]` has two, the
packet store at `141e36bbc` and the read at `141e4a6ce`. **This function is their only
consumer.** [L]

So the right name for bytes 12..19 is **`zLayer` / `zWithinLayer`, a per-NPC draw-order
override**, not "anim-time override A/B". [D]

---

## 4. The negative, re-established with a different instrument

`npc-appear.md` swept the CNpc range for `call [reg+0x200]` and checked the nearest
preceding `mov edx, imm32`. That is a search of a known range for a known shape. I asked the
**inverted** question over the **whole image**, which is a different question, not a re-run:

> Find every `call qword ptr [reg+0x200]` in `.text`, bound each one by its `.pdata`
> function, disassemble that function with resync, and report the ones whose `edx` is
> **computed** rather than immediate.

`research/msexe-putcolor-sweep.txt`. Result: **838 call sites, 710 immediate-fed, 122
computed, 6 unresolved.** Of the 122 computed, **zero are in `0x141e3xxxx..0x141e7xxxx`.**
**[L]**

The instrument's positive control is inside its own output: it finds `sbb edx,edx`,
`cmovne edx, ecx`, `or edx, eax`, `dec edx` and `mov edx, [rbx+0x14a8]` forms elsewhere in
the image, including the `neg esi; sbb edx,edx` stub at `0x141b096a0` that `npc-appear.md`
names — so it can see computed alphas, and it is not returning an empty CNpc set by
construction.

A second cut, filtered by IID so every row is provably a call on *this* interface
(`research/msexe-layer-iface-calls.txt`): **[L]**

| | `+0x200` (`put_color`) values |
|---|---|
| CNpc | `0xFFFFFFFF` x14, `0xFFFFFF` x1 — opaque or invisible, nothing between |
| CMob | `0xFFFFFFFF` x18, `0x80FFFFFF` x2, `0xFF808080` x1, `0xFFFFFF` x1 |

Note the direction of that: the class that **does not** fade (CMob) is the one with
half-alpha constants. CNpc has none.

The same table enumerates **every** layer method each class calls — which closes
`npc-appear.md`'s own named gap, *"an alpha ramp ... reached through a vtable slot other than
`+0x200`"*. CNpc calls only three slots CMob does not: `+0x90`, `+0x228`, `+0x408`, once
each. CMob calls fourteen CNpc does not. There is no NPC-only slot with the call count or
the argument shape of a ramp. **[L]**

And the per-frame gate: the one non-constant `put_color` in CNpc's update virtual
(`FUN_141e3e220`) is decided by `FUN_14158f2f0`, which is four instructions —
`xor eax,eax; cmp qword [rcx+0x18], rax; setne al; ret`. A null test. Binary. **[L]**

---

## 5. What the mob's fade actually was, and why it has nothing to transfer

`npc-appear.md` read the mob's `appearType = -1` and `-2` cases. The case that mattered is
the **default** (`appearType >= 0`), `0x141c52a95`, which is what the server used to send:

```asm
141c52a9f  mov  rdx, [0x143a48f68]         ; -> L"Effect/Summon.img/%d"
141c52aad  call 0x1401c21c0                ; format it with r8d = appearType
141c52ad1  mov  ecx, [r14 + rax*4 + 0x1c5403c]   ; a second table, appearType 0x54..0x59
141c52ade  mov  edi, 0x960                 ; 2400 ms
141c52ae5  mov  edi, 0x4b0                 ; 1200 ms
141c52aea  mov  r14, [0x143a45cd8]         ; -> L"delay"
141c52b1d  mov  [rsi+0x304], ecx           ; duration
141c52b44  mov  [rsi+0x300], r15d          ; appearType
141c52b4b  mov  [rsi+0x504], edi           ; = 1
```

Both globals resolve to UTF-16 literals I dumped from `.rdata`: `Effect/Summon.img/%d` at
`0x1432B3BF8` and `delay` at `0x14328C9E0`. **[L]** So the mob's pre-fix "fade" was a
**separate summon-effect object with a duration**, and `appearType = -1` skips creating it.
It was never an alpha ramp on the mob itself, which is why nothing about it transfers to an
object that has no such field.

`tools/dataref.py 0x143a48f68` gives **six readers**: `0x14184a570`, `0x141c4ff80` (the mob
decoder), `0x141c626c0`, `0x141cc2d90`, `0x141d32b30`, `0x141d3b1f0`. **None is in the CNpc
range.** NPCs have no summon effect. **[L]**

---

## 6. The one structural fact that ends the question

`FUN_141cb9fd0` (`0x141cb9fd0 .. 0x141cba9f6`, 2598 bytes) is `CNpc::FUN_141e4a5d0`'s twin
for mobs, and the decompilation is in `research/msexe-npc-clocksync.c`. Side by side: **[L]**

| | CNpc `FUN_141e4a5d0` | CMob `FUN_141cb9fd0` |
|---|---|---|
| layer field | `[npc+0x238]` | `[mob+0x610]` |
| displayer | `[npc+0x160] - 0x20` | `[mob+0x2b8] - 0x20` |
| secured pair | `FUN_1409c6cc0` / `FUN_1409c6ce0` | the same two functions |
| formula | `(A*3000 - B)*10 - 0x3FFF8AD5` | `(A*3000 - B)*10 - 0x3FFF8ADA`, then `+1` |
| setter | `vtable + 0x198` | `vtable + 0x198` |
| error IID | `DAT_14327fcb0` | `DAT_14327fcb0` |
| `0xC0000000` special case | 6 template ids | ~24 template ids |
| called from the decoder | `0x141e389f2`, unconditional | `0x141c52b80`, unconditional, **after** the `appearType` switch converges |

The mob runs this on **every** spawn, including the `appearType = -1` spawns that the owner has
confirmed appear instantly. **Whatever `vtable+0x198` is, it is not what makes an NPC fade.**

---

## 7. WIRE IT LIKE THIS

**There is nothing to wire.** No byte of `npc_enter_field` selects an appear or fade mode,
and the one call the previous pass could not rule out is now ruled out by the mob doing the
identical thing.

Two things the coordinator should nevertheless know, because they are live hazards:

1. **Bytes 12..19 are a draw-order override, and getting them wrong makes NPCs vanish.**
   Sending `-1`/`-1`, which is what the server does today, is correct: it is the
   constructor default and it skips the block entirely, leaving the client's own computed z
   in place. If anyone sends `0`/`0` "to see what happens", the NPC is placed at
   `z = layer 0, offset 0` — **behind the map's own background tiles on most maps** — and it
   will read on screen exactly like the `isEnabled = 0` failure that hid every NPC for days.
   This is the failure mode the brief asks to be stated: **NPCs stop appearing at all.**
   Do not touch these bytes.
2. `crates/net/src/opcode.rs`'s comment on bytes 12..19 should say
   **`z layer` / `z within layer`, `-1` = client default**, not "anim-time override". I have
   not touched the file; it is the coordinator's. This is a comment change, not a byte
   change.

`npc-appear.md` §5's two recommendations stand unaltered by this pass: byte 58..61 is not
`ALPHA`, and bytes 20/21 look swapped. Neither is the fade.

### The client-patch route

**Not viable today, and the reason is not squeamishness — there is no address to patch.**
`docs/launcher.md` records five runtime patches, every one of them a single absolute VA in
`MapleStory.exe` with a named retirement condition. A patch needs a located instruction.
This pass and the previous one between them have shown that **no instruction in CNpc
computes a partial alpha**, so there is nothing in the client image to `ret` out or to
rewrite. The only place a ramp could still live is inside `Gr2D_DX11.dll` (§8), which the
current patch machinery does not address at all — it resolves VAs against the client image,
not against a loaded module — and a patch there would enter the inventory with a blank
retirement column, which `docs/launcher.md` explicitly argues against.

---

## 8. What is still open, stated as a blind spot rather than a result

**I did not decompile `Gr2D_DX11.dll`.** It is not in the Ghidra project, and my attempt to
locate the concrete layer vtable statically failed: the ATL interface map at rva `0x44eb10`
gives the sub-object offset (`0x10`) but no vtable, the 49 `lea` references to the IID all
land in ordinary consumer code rather than a `QueryInterface` at vtable slot 0, and the DLL
carries no RTTI for the class and no `MSFT` typelib.

So the honest residual is the same one `npc-appear.md` named, narrowed by one function: **an
alpha ramp inside the Gr2D layer implementation, triggered by how the layer is created rather
than by anything CNpc computes.** The candidate is the layer-insert flag: CNpc calls
`+0x280` twenty times with `mov edx, 0x20` (x13) and `xor edx, edx` (x7); CMob calls it
ninety-two times with `xor edx, edx` (x59) and `mov edx, 0x20` (x30). Same two values, very
different ratio. **[I] — I am naming it, not claiming it.**

Also unproven, and worth saying because it is cheap to settle and nobody has: **that what
The owner sees is an alpha fade at all.** Nothing in the client image can produce a partial-alpha
NPC. An object that arrives late, or that is drawn while the map's own transition is still
running, reads on screen the same way.

---

## 9. The single client test

The test is `npc-appear.md` §6's, unchanged, and I am endorsing it rather than inventing a
worse one — but this pass makes it **strictly more decisive**, because CNpc is now eliminated
as a source of a ramp, so the outcomes partition cleanly.

**One variant. One observable. No existing byte changes.**

### What to change

In `crates/world/src/session/field.rs`, leave `on_field_entered` exactly as it is and send
**one extra `0x044F`** on the existing tick, **3 seconds after field entry**:

* `template_id` = `1` (Heena — art already loaded on map 1)
* `object_id` = `9001` (cannot collide with the real two)
* `x` = Heena's `x + 180`
* **every other byte identical to what `npc_enter_field` already produces**, bytes 12..19
  included, i.e. still `-1`/`-1`

The coordinator owns that file; I have not touched `crates/`.

### What the owner does

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

Log in, enter map 1 (Mushroom Town — West Entrance), then **stand still and watch the empty
ground to Heena's right for about five seconds.** A second Heena will appear there. Report
**how** they arrive — and specifically whether they are ever **see-through** (the ground
visible through them) as opposed to simply **not there and then there**.

### What each outcome means

| on screen | meaning |
|---|---|
| **They are see-through for a moment, then solid**, three seconds after the map has settled | The ramp is real and it is a property of **creating a layer**. Since no instruction in CNpc computes an alpha (§4), it can only be inside `Gr2D_DX11.dll` (§8). That is the next and last place to look, and it is a **client-side, unconditional** behaviour — the server cannot reach it |
| **They pop in instantly, solid from their first frame** | The fade is **not** a property of creating an NPC. It is scoped to the field-entry moment, which the server owns: `on_field_entered` fires on `0x00DC`. The next step is *when* the packets are sent, not *what is in them* |
| **They are absent for a beat and then simply appears, never see-through** — and the same is true of the map-entry NPCs | It was never a fade. It is a late first draw, and the question changes from "which field controls alpha" to "why is the first draw late" |
| **Nothing appears at all** | The extra send is broken, not the body. `world.log` names every `0x044F` that went out; if the third is not in the log the server never sent it. If it is, suspect the object id taking the pool's "already present" path (`research/npc-spawn.md` §5) |
| **The two map-entry NPCs stop appearing** | The edit touched more than it should have. This is the `isEnabled` failure mode and it is why the test is specified as *add a packet, change no existing byte* |

The first three rows are the informative ones and they are mutually exclusive. Rows four and
five are self-diagnosing from `world.log` without a second launch.

---

## 10. Instruments, controls, and what each one could not see

| instrument | control run | blind spot, quoted or named |
|---|---|---|
| Ghidra headless, `DecompileFunc.java` | three functions in one run; output matched the linear listing line for line on `FUN_141e4a5d0` | *"the decompiler lies about order; the listing does not"* — every field offset quoted here comes from `tools/listing.py`, the meanings from the decompiler |
| `tools/listing.py` | its output for `FUN_141e4a5d0` reproduces the same 1014-byte extent `pdata_lookup.py` gives | shares `reads.py`'s loader, so it cannot disagree with it about extents — a guarantee, not a check |
| `tools/fieldrefs.py` | `0x5a8` returned the constructor default, the packet store and the single consumer — three hits, the shape its docstring predicts | *"`rsp`/`rbp`/`rip`-based operands are dropped"*. It did **not** bite this time: every `0x5a8`/`0x5ac` reference is `[reg+disp]`. `--write` was deliberately **not** used, per `CLAUDE.md`'s *"a `[reg+disp]` write-scan cannot see a store through a pointer that was `lea`'d and handed off"* |
| whole-image `put_color` sweep (new, `research/msexe-putcolor-sweep.txt`) | 838 sites, 710 immediate, **122 computed** including `sbb edx,edx` and `cmovne` — so it can see computed alphas | matches `call [reg+0x200]` on **any** interface, not only this one; it cannot see `mov rsi,[rax+0x200] ... call rsi`, which is the form `FUN_141e36b20` uses for slot `+0x118` |
| IID-confirmed vtable-slot enumeration (new, `research/msexe-layer-iface-calls.txt`) | reproduced both call sites I already knew by hand — `141c51a24` (mob, `0x3e8`) and `141e4a63a` (NPC) | only sees a call whose `_com_issue_errorex` follows within 48 bytes; a call whose HRESULT is ignored is invisible. Rows with `disp > 0x1000000` are resync noise and are labelled as such in the file |
| capstone linear sweeps | **this one failed first and was caught.** `md.disasm` stops at the first undecodable byte; my initial Gr2D scan reported 1698 `lea`-rip instructions in a 4 MB `.text` and 0 references to the IID. With a resync loop the same scan reports 10 742 and **49**. Every Gr2D conclusion drawn before the fix was discarded | resync produces garbage instructions in data; only used for presence, never for absence, after that |
| `tools/xref.py` | found `Etc/NpcNoticeBoard.img/%d` at the two `lea` sites my own string walk found | *"a 0 references result means nothing takes its address"* — and it **bit**: `Npc/%07d.img` has **0** `lea` references because the client reaches it by a pointer read. So an absence of `Effect/...` in a CNpc string list is **not** evidence, and I did not use one as such (§5 uses `dataref.py` on the global instead) |
| `tools/dataref.py` | six readers for the `Effect/Summon.img/%d` global, one of them the mob decoder I already knew | reads/writes/tests of a global; not a call |
| `tools/rtti.py` | listed the one `Gr2D`-matching descriptor in msexe | `Gr2D_DX11.dll` has 158 type descriptors and **none** names the layer class |
| `tools/callers.py` | 5 call sites plus **1 tail `jmp`** for `FUN_141e4a5d0`, 8 plus **2 tail `jmp`** for `FUN_141cb9fd0` — the tail-jump half is exactly the 27 909-function hole `CLAUDE.md` records | still cannot see a call through a vtable, which is how both are mostly reached |

Not used: `tools/encodes.py` (misses loop-written fields, still unfixed).
`ModernMapleSource` was **not consulted at all** for anything in this document.
