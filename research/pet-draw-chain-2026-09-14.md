# The pet's draw chain, read out of CPet::Init - 2026-09-14

Written after `-PetSync` proved the client has the pet **marked visible** and draws nothing
(`research/pet-dead-is-datedead-2026-09-14.md` §7). The question is no longer "is it shown"
but "is there anything to show", and this is a Ghidra pass at that question. **No client run
was spent.**

Sources produced by the pass: `research/msexe-pet-init.c` (`CPet::Init` `FUN_141eb9760`, the
template cache `FUN_141ed3540`, the periodic updater `FUN_141ebfa30`, the sync `FUN_141ecde00`
and its sibling `FUN_141ecdff0`, and `FUN_14159b0a0`), `research/msexe-pet-action.c`
(`FUN_141ec7e90`, `CPet::SetStance` `FUN_141ec7880`, `FUN_141ec87b0`, the WZ template loader
`FUN_141ed8f70`) and `research/msexe-pet-frames.c` (`FUN_141ecaa40`, `FUN_141eca710`).

Labels: **[L]** read off the client · **[D]** derived · **[I]** inferred.

---

## 1. The chain, in order **[L]**

```text
CPet::Init  FUN_141eb9760
  pet[0x21], pet[0x22]  <- FUN_141ed3540(itemId), the template cache. EITHER null -> Init
                           returns 0, so both loaded: the pet exists, so this passed
  pet+0x3b8 (pet[0x77]) <- DAT_143add050->vtbl[0x1d8](0, &out)     the SPRITE
  pet+0x3c8 (pet[0x79]) <- <factory>->vtbl[0x168](11 args, 5 VARIANTs)   the LAYER
                           this is the object -PetSync measured as visible
  layer->vtbl[0x238](VARIANT(VT_UNKNOWN, sprite))                  sprite into layer
  FUN_141b054f0(pet, pet[0x26], &layer, ..., 0x3eb, ...)           layer into the render
  FUN_141ec7880(pet, FUN_141ec7e90(pet), 1)                        CPet::SetStance
  FUN_141ec87b0(pet)
  FUN_141ecaf00(pet)                                               the NAME TAG
```

The name tag is built here, at construction - which is why it draws even though nothing else
does, and why it says nothing about the visibility sync.

## 2. Where frames actually enter, and the gate nobody has measured **[L]**

`CPet::SetStance` (`FUN_141ec7880`) stores the stance obfuscated at `pet+0x370` and then forks:

```c
if (stance == 0) { FUN_141ec2690(pet); return; }   // the LAND arm - returns here
... otherwise the flying/second path: FUN_141ec22f0(pet, &list, ..., 3), layer->vtbl[0x200] ...
```

`FUN_141ec2690` picks one of three arms on `pet[0x7d]` / `pet[0x7b]` / `pet[0x77]`, and the
`pet[0x77]` arm - the one a normal pet takes - ends in `FUN_141ecaa40`, which is where frames
go in:

```c
void FUN_141ecaa40(pet, x, VARIANT *v) {
    if (*(void**)(pet + 0x3d8) != 0) { clear(v); return; }      // <- SILENT BAIL
    sprite = *(void**)(pet + 0x3b8);
    sprite->vtbl[0x20]();                                        // reset
    sprite->vtbl[0x40](&v);                                      // the frames
    layer  = *(void**)(pet + 0x3c8);
    layer->vtbl[0x238](VARIANT(VT_UNKNOWN, sprite));             // sprite into layer
}
```

**`pet+0x3d8` non-zero means the pet's frames are never inserted, nothing is logged, and every
COM call that did run returned S_OK.** That is exactly the shape of the bug on screen: a
registered, positioned, visible, empty layer. Nothing has measured that field, and this note
does **not** claim it is set - only that it is a gate on the one function that would fill the
layer, and that it is cheap to read.

## 3. A lead that died inside the pass

Worth writing down so it is not rediscovered: `moveAction` is stored obfuscated at `pet+0x2d8`,
and `FUN_141ebe350` decodes it as **bit 0 = facing** and `(moveAction >> 1) - 1` indexed into a
fifteen-entry jump table at `0x141ebe408` **[L]**:

```text
moveAction   2/3  4/5  6/7  8/9 10/11 12/13 14/15 16/17 18/19 20/21 22/23 24/25 26/27 28/29 30/31
stance         0    1    3    0     0     4     0     0     0     0     2     5     6     7     8
moveAction 0 or 1 -> index -1 -> unsigned 0xffffffff > 14 -> the default arm -> 0
```

and `pet+0x308` overrides the result when it is `>= 0`.

It is tempting to conclude that `moveAction = 0` - which this server sends - drops the pet off
the table and into stance 0, and that stance 0 is why nothing draws. **It does not follow.**
`FUN_141ec7e90`, the only caller that matters here, returns a **bool**:

```text
141ec7e90:  if (pet+0x120 == 0) return 0;              the owning user
            if (FUN_14276e860(user) != 0) return 1;
            if (FUN_141ebe350(pet, 0) == 8) return 1;  only moveAction 30 or 31 decodes to 8
            return 0;
```

so the value handed to `SetStance` is 0 or 1 and nothing else, and 8 is a special action rather
than the ordinary case. **Stance 0 is the normal land path and `moveAction = 0` is correct.**
Changing it would send a land pet down the second arm. Do not.

## 4. Also checked, also not it

* **The WZ path.** `FUN_141ed8f70` builds `Item/Pet/%07d.img` (`0x1432acd30`), which for
  5000006 is `Item/Pet/5000006.img` - exactly the node our archive has **[L]**.
* **The template.** `FUN_141ed3540` is a hash-map cache keyed on the item id with a lazy load;
  `CPet::Init` returns 0 if either of its two lookups comes back null, and the pet exists, so
  both resolved **[L]/[D]**.
* **`FUN_141ecaf00`** is the name tag, not the sprite: it rebuilds `pet+0x3d0` from
  `pet+0x138` behind a twelve-character length check, and `tools/callers.py` shows `CPet::Init`
  among its six callers **[L]**.

## 5. The next measurement

`-PetFrames` in `tools/test-server.ps1`. Three watches down the chain plus the positive
control, and the deepest one that fires names where it stops:

```text
141ec7880:hits=2000            CPet::SetStance - rdx is the stance, r8 the force flag
141ec2690:hits=2000            the land arm
141ecaa40:peek=3d8:hits=2000   the frame insert, and the peek reads the exact field its
                               early bail tests
140304100:hits=200             positive control
```

Two outcomes end this: `141ecaa40` with **peek non-zero** makes `pet+0x3d8` the bug, and
`141ecaa40` with **peek 0** eliminates the entire chain - frames went into a visible layer and
it still drew nothing - which would move the hunt to the sprite's own contents.
