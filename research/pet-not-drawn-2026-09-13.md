# The Husky is summoned and does not draw - 2026-09-13

The owner, with a screenshot: *"I have the Husky summoned, but my client does not render it."* The name
tag **"Husky"** is drawn beside **"Wisp"**, at the same height, a little to the left.

## 1. What that screenshot already proves

The name tag is the pet object's own. So the packet was accepted, the object was built, it was
registered on the user, and it is positioned where we put it. Everything the server sends is
therefore right, and this is a **client-side visibility decision**, not a packet bug. Checked
one by one:

| could it be | no, because |
|---|---|
| the packet mis-parsed | the name renders as "Husky" and the tag sits at the character's feet - `itemId`, `name`, `x`, `y` all decoded correctly, so the field order is confirmed on screen |
| the art is missing | `Item/Pet/5000006.img` has 22 action nodes (`stand0`, `move`, `jump`, ...) and `Item/Pet/_Canvas/_Canvas_000.wz/5000006.img/stand0/0` is a real **41x37** bitmap |
| our installer broke the pet image | the patch only adds `info` leaves; `stand0/0` still carries its `_outlink`, and the canvas archive is untouched |
| the template failed to load | `CPet::Init` (`FUN_141eb9760`) **returns 0** unless both `FUN_141ed3540` template loads succeed, and a 0 return means the handler never registers the pet - there would be no name tag |
| the foothold is wrong | we sent `fh 43` on map 1010; `gm-handbook/footholds.txt` has 114 footholds there including 43, and NPCs use the same WZ-id convention (`net::opcode` NPC read 7) and draw |
| the character record must list the pet | the classic stat block is 108 bytes and **fully enumerated** in `research/charstat-layout.md` - it has no pet-serial array (the v83 `petSN[3]` is not in this build) |
| `init = 1` hides it | the `init == 0` branch in the local handler only builds a **message** from string ids `0x1a6..0x1a8`; it does not touch the sprite |

## 2. What actually decides whether a pet draws

**`FUN_141ecde00(pet)`** - called from the activation handler and again from `SetPet`
(`FUN_1427707e0`). It is the pet's show/hide, and its tail is unambiguous **[L]**:

```c
if (verdict != current) {
    pIVar1->vtbl[0x2b8](pIVar1, verdict);   // the renderable's visibility
    FUN_14159b0a0(pet[8], verdict);         // <- rdx IS the verdict
    pet->vtbl[0x18](pet, verdict, 0);
    FUN_141ecaf00(pet);
    if (verdict == 0) { release pet[0x20]; }
}
```

`verdict` starts at **0** and becomes 1 only if every gate in this chain passes, in order:

```text
FUN_1409d6150(pet+0x630)        must be 0
pet[0x24] != 0                  the owning CUser  (set at Init line 158; a non-zero
                                vtbl[0x58] on it makes Init return 0 outright)
pet[0x23] != 0                  a COM interface from FUN_142b5a560 -> QueryInterface
FUN_142826340()                 must be 0   - the morph/transform test (research/quest-complete-effect.md)
FUN_140f80830(user+0x100)       must be 0
FUN_140f80860(user+0x100)       must be 0
  (if user->vtbl[0x50]() == 0 and FUN_142d0f360(field) != 0 -> jump to the end, stays hidden)
FUN_1409bd2f0(pet[0x23]-0x20)   must be 0
FUN_141892840 / FUN_14183a640   the local-user test -> bVar2
user->vtbl[0x50]() != 0 || !bVar2
FUN_142cc1e40(field)            must be 0
                                -> verdict = 1
```

**Every gate is the state of the USER and the FIELD at that instant, not of our packet.** None
of them can be evaluated from the file, so this is where static analysis ends.

## 2b. The run of 19:21 - the pet is hidden ~30 times a second, permanently

Four watches, all armed and int3-verified. What came back:

```text
141ecde00   40 hits, the FIRST at 19:21:07.571 - the instant of the summon (0x0277 at 23:21:07.569)
            called-from  0x1428a027e   1   the local pet-activated handler
                         0x142770819   1   SetPet
                         0x141ec1ce8  38   a periodic updater, ~30 ms apart
14159b0a0    3 hits, ALL from 0x141e4c056, all at 19:21:04.1 - before the summon, nothing to do with the pet
1409bd2f0   40 hits, cap reached 19:21:05.159   } both exhausted BEFORE the summon,
142cc1e40   40 hits, cap reached 19:21:04.380   } so they say nothing about the pet
```

Two things follow, and the first is the important one.

* **The evaluator ran for the pet and only for the pet** (its first hit is the summon), and then
  **38 more times from a periodic updater**. On every one of those 40 evaluations it did **not**
  call `FUN_14159b0a0`, and that call only happens when the verdict differs from the current
  state. So the verdict was "hidden" at the summon and stayed "hidden" on every frame after.
  **This is a steady-state refusal, not a race** - which also means walking a portal cannot help,
  because the client is already re-asking the question thirty times a second.
* The two mid-ladder watches burned their 40-hit caps on unrelated callers seconds before the
  summon. That is the instrument's fault, not a result; the next run raises the caps.

## 2c. Three gates settled without a launch

`tools/dis_at.py` on each gate, and the ladder's own disassembly (`tools/listing.py 0x141ecde00`),
settle three of them outright **[L]**:

| gate | call site | function | verdict |
|---|---|---|---|
| 6 | `141ecde94` | `FUN_140f80860` | **always passes** - the function is `call get_local_user; xor al, al; ret`. It cannot return non-zero |
| 7 | `141ecdea7` | `user->vtbl[0x50]` = `FUN_142889020` = `mov eax, 1; ret` | **always 1**, so the `jne` at `141ecdeac` is always taken and gate 7's `FUN_142d0f360` is never reached |
| 10 | `141ecdef8` | the same `vtbl[0x50]` | **always 1**, so the `jne` at `141ecdefd` is always taken and the `bVar2` test at `141ecdeff` is dead. The local-user pair `FUN_141892840` / `FUN_14183a640` computes a flag nothing reads |

So the surviving suspects are **five**: gate 1 `FUN_1409d6150(pet+0x630)`; gate 4
`FUN_142826340` (`FUN_141715f80` morph, then `FUN_140fb0030(user+0x100)`); gate 5
`FUN_140f80830`, which is a tail call to `FUN_14182ffd0(localUser)`; gate 8 `FUN_1409bd2f0`;
and gate 11 `FUN_142cc1e40(field)`, which is two flags:

```asm
142cc1e40  cmp dword [field + 0x24b0], 0   ; either non-zero
142cc1e49  cmp dword [field + 0x24ac], 0   ; -> returns 1 -> pet hidden
```

## 3. The measurement that names the gate

The ladder short-circuits: **every gate that fails jumps to `0x141ecdf15`**, the stay-hidden
label. So the last of the five suspects that is entered *from inside the ladder* names how far
execution got, and the gate after it is the one that closed. The probe logs `called-from`, which
is what separates a ladder call from the same function's many other callers:

| watch | its `called-from` inside the ladder | if it appears |
|---|---|---|
| `1409d6150` | `0x141ecde27` | gate 1 was reached (it always is) |
| `142826340` | `0x141ecde63` | gates 1..3 passed |
| `1409bd2f0` | `0x141ecdec6` | gates 4..7 passed - so if this is absent while gate 4 appeared, **gate 4 or gate 5** is the blocker |
| `142cc1e40` | `0x141ecdf10` | gates 8..10 passed, so **gate 11** is the blocker |

Plan step TO(v), second run. **The caps must be large** - last time two of them were spent on
unrelated callers within a second of entering the field - and the pet should be summoned
immediately, because the ladder then runs about thirty times a second and any depth is sampled
hundreds of times.

## 4. Not changed, and why

Nothing. The one field where this server knowingly differs from the reference is the **hue**: it
sends `0` where `Pet.encode` annotates `-1`. That is a candidate, but it is a guess, the gauntlet
above is the measured mechanism, and a guess costs the same launch as the measurement.
