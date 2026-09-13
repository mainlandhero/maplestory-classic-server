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

## 3. The measurement that names the gate

The chain short-circuits, so **the last predicate that is entered names the gate that closed.**
Four watches read it, and `FUN_14159b0a0`'s `rdx` is the verdict itself. It is only called when
the verdict *changes*, so no hit at all means the pet was created hidden and stayed hidden.

```text
watch@141ecde00   the evaluator ran at all
watch@1409bd2f0   execution reached the second half
watch@142cc1e40   execution reached the last gate
watch@14159b0a0   rdx = 1 shown, rdx = 0 hidden; absent = never left hidden
```

Plan step TO(v). If `14159b0a0` fires with `rdx=1`, the pet was told to show and the fault is
downstream of visibility (the animation or the draw order), which is a different hunt.

## 4. One thing worth trying in the same run, free

Walk through a portal. `Session::pet_entry_replies` re-sends the activation on every field entry,
which runs the whole evaluator again against a fresh field. If the pet appears after a map
change, the verdict was state-dependent at summon time and the fix is *when* the server sends
the activation, not what is in it.

## 5. Not changed, and why

Nothing. The one field where this server knowingly differs from the reference is the **hue**: it
sends `0` where `Pet.encode` annotates `-1`. That is a candidate, but it is a guess, the gauntlet
above is the measured mechanism, and a guess costs the same launch as the measurement.
