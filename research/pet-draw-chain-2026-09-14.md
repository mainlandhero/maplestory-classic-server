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


---

# The `-PetFrames` run: the chain executes, and the layer is not empty

`research/fixtures/pet-frames-inserted-and-still-blank-3d8-was-zero-hook.log`, 2026-09-14
17:07:07. This is the outcome §5 pre-registered as the one that **eliminates the chain**.

```text
140304100    36 hits   POSITIVE CONTROL - the hook armed
141ec7880  1166 hits   CPet::SetStance. The summon call is from Init (called-from
                       0x141ebbe86) with rdx=0, r8=1: stance 0, forced - exactly what
                       the static read predicted, so moveAction 0 is CONFIRMED correct
141ec2690     3 hits   the land arm, all from 0x141ec7e65 inside SetStance
141ecaa40    11 hits   and [pet+0x3d8] = 0x00000000 on EVERY one
```

The bail never fired. The sprite was reset, given its frames and inserted into the layer -
eleven times, from three different call sites (`0x141ec29b0` in the land arm, `0x141eca9fc`
in `FUN_141eca710`, `0x141ec2523` in `FUN_141ec22f0`). Combined with `-PetSync`: **the sprite
exists, has frames, sits in a registered, positioned, visible layer, and nothing draws.**

Sections 1 and 2 of this note are therefore closed. They were right about the mechanism and
the mechanism is not the fault.

## 6. The one lead left: a second writer to the pet's enable flag

Found reading `CPet::SetStance` for that run rather than from a new pass. Every periodic call
- force 0, stance unchanged, and there were 1163 of them - falls through to **[L]**:

```text
141ec794a  mov  rcx, [rbx+0x120]     ; the owning user
141ec7951  add  rcx, 0x100
141ec7958  call 140f8abc0            ; xor eax,eax / cmp [rcx+0x5ac],eax / setne al
141ec795f  je   epilogue             ; zero -> nothing happens at all
141ec7965  mov  rax, [rbx]
141ec7968  xor  r8d, r8d
141ec796b  xor  edx, edx
141ec796d  mov  rcx, rbx
141ec7970  call [rax+0x18]           ; pet->vtbl[0x18](pet, 0, 0)
```

**`vtbl[0x18]` is the same slot the visibility sync writes.** `FUN_141ecde00`'s show path is
`pet->vtbl[0x18](pet, verdict, 0)`. So two things write that flag and the periodic one always
writes **zero** - and the sync can never correct it, because what the sync reads back is the
**layer's** flag through `[pet+0x3c8]->vtbl[0x2b0]`, a different object. That is exactly why it
measured "already visible" 2964 times in a row while nothing was on screen.

**This is a hypothesis and it has a cheap way to be wrong.** The call only happens when
`user+0x100+0x5ac` is non-zero, and no run has ever read that field. `-PetEnable` reads it with
`140f8abc0:peek=5ac`, watches `141ec7970` for the call itself, and adds `140eeba60` - the
layer's colour setter, which `SetStance` can hand `0xffffff` (alpha 0, fully transparent) for a
pet that is not the user's current one. That arm needs `stance != 0` so it should be silent;
the watch is what turns "should" into "is", and transparency-with-perfect-layout is a shape
this project has already been caught by once, in the NPCs.

Not established, and deliberately not written as a candidate: what sets `user+0x100+0x5ac`.
`FUN_140f810e0`, the user-state setter the `-UserState` probe already watches, writes
`+0x5e4` on the same object - a **different field**, so the two are not the same thing.



---

# The `-PetEnable` run: the hypothesis is dead, and so is transparency

`research/fixtures/pet-enable-flag-never-zeroed-and-layer-is-opaque-hook.log`, 17:17:17.

```text
140304100    36 hits   POSITIVE CONTROL - the hook armed
140f8abc0  6000 hits   [user+0x100+0x5ac] = 0x00000000 on EVERY sample. The gate is shut
141ec7970     0 hits   so the disable call cannot fire, and it did not
140eeba60  2950 hits   1281 of them carry 0xffffff - alpha 0, fully transparent
```

The 1281 transparent calls looked alarming for about a minute. **None of them is the pet.**
Correlating by the layer pointer that `SetStance`'s own call passes (`rcx=0x3081d910`), the
pet's layer appears in that log **exactly once**, from `0x141ec794a`, with `rdx=0xffffffff` -
opaque. The other calls are other objects.

So §6 is dead on its own pre-registered terms, and transparency with it.

## 7. Where this leaves it

Five separate captures, five things measured working:

| | measured |
|---|---|
| the item is alive | tooltip reads 1/1/2077, `dateDead` fixed |
| the sync says VISIBLE | 2964 gate-11 reads of 0, and no transition, so current == desired == 1 |
| the frames go in | `141ecaa40` x11, `pet+0x3d8` = 0 every time |
| the enable flag is never zeroed | `user+0x100+0x5ac` = 0 on 6000 reads; `141ec7970` never fired |
| the layer is opaque | one colour write, `0xffffffff` |

The sprite and the layer both come from `DAT_143add050`, the client's own Gr2D root - the same
singleton every other drawable uses. The packet, the WZ, the template, the position and the
name tag were eliminated earlier.

**The pet is built correctly and is not on screen.** Running the same kind of instrument again
is the failure `CLAUDE.md` describes - widening the input to a tool with a structural blind
spot only makes the blind spot bigger, and a second opinion means changing the question.

The question this has never asked is what the layer is **attached** to:

```text
141ebbe68  call FUN_141b054f0(pet, pet[0x26], &layer, ..., 0x3eb, ...)
```

`pet[0x26]` is `pet+0x130`. Null there would attach the layer to nothing while every single
measurement above still reads perfect - which is exactly the situation. `-PetLayer` reads it
at the registration (`rdx`, with `called-from=0x141ebbe6d` identifying the pet's call in a
shared 16 KB function) and again 1166 times over the pet's life through
`141ec7880:peek=130`, so a field that is set at Init and cleared later cannot hide either.

If that comes back clean too, the honest next move is a Ghidra pass on `FUN_141b054f0`
itself, not another launch.



---

# The `-PetLayer` run and the `FUN_141b054f0` pass: the layer IS drawn

`research/fixtures/pet-layer-attachment-is-fine-name-is-pet0x26-hook.log`, 17:30:11.

```text
140304100    36 hits   POSITIVE CONTROL
141b054f0    35 hits   ONE of them from the pet (called-from 0x141ebbe6d), rdx =
                       0x3fda6968, and the deref shows 0x6b737548 = "Husk"
141ec7880   690 hits   [pet+0x130] = 0x3fda6968 on every one - steady, never cleared
141ec87b0    86 hits
```

`pet[0x26]` is the pet's **name**, not a parent, and the registration happened. Attachment
is fine, which was the pre-registered "out of structural leads" outcome - so the promised
Ghidra pass on `FUN_141b054f0` followed, and it changed the picture twice.

## 8. `0x3eb` is the NAME TAG, and that means the layer is drawn

`FUN_141b054f0`'s `param_5` is the `0x3eb` Init passes, and it selects a format string
**[L]**:

```text
141b0578x   if (param_5 == 1000 || param_5 == 0x3f2 || param_5 == 0x3eb)
                puVar28 = PTR_u_UI_NameTag_img_pet__d_143a46ba0    // "UI/NameTag.img/pet/%d"
            FUN_1401c21c0(&local_178, puVar28, param_6);
```

so that call **builds the pet's name tag**, and Init hands it `pet+0x3c8` - the pet's own
layer, AddRef'd into `local_138` first - as the layer to build it in. `param_3` is in/out and
the function only creates a layer when `*param_3 == 0`, which it is not here **[D]**.

**The tag is on the owner's screen.** So the pet's layer is attached to the render tree, visible,
and being drawn - proved by its own child rather than by a flag. The missing thing is only
the sprite inside it. That is a much smaller target than "the pet does not draw", and it
came out of a screenshot and a decompile with no launch.

## 9. The pet has its own alpha, and nobody has read it

The tail of `CPet::Init`, at `0x141ebc1b2` **[L]**:

```text
141ebc1a1  mov  eax, 0x51eb851f
141ebc1a6  imul ecx                       ; ecx = *(int *)(DAT_143ac87a0 + 0x58)
141ebc1a8  sar  edx, 5                    ; edx = ecx * 255 / 100
141ebc1b2  mov  dword [rdi + 0x3c0], edx  ; <- the pet's ALPHA
141ebc1bf  mov  rbx, [rdi + 0x3b8]        ; the sprite
141ebc1dd  call [rax + 0x68]              ; sprite->vtbl[0x68](table[branch])
```

`DAT_143ac87a0 + 0x58` is a global config **percentage**; `+0x5c` is the other arm and is
unreachable, because reaching it needs `user->vtbl[0x50]` to return 0 and that function is
`mov eax,1; ret`. The whole block is skipped when `DAT_143ac87a0` is null.

**A zero percentage there gives alpha 0: an invisible sprite inside a layer that still draws
its name-tag child.** That is precisely the screen. It is a *different* mechanism from
`FUN_140eeba60`, the layer colour, which `-PetEnable` already cleared by pointer. `pet+0x3c0`
is written once here and read by nothing in any pet function dumped so far, so the render
consumes it.

**Not established:** what that config is meant to hold, or who fills it. `-PetAlpha` reads
the value at the instant it is written (`141ebc1b2`, where `rdx` *is* the number) and again
across the pet's life (`141ec7880:peek=3c0`), and the run can come back `0xff` and kill it.



---

# `-PetAlpha` and `-PetParent`: alpha is fine, and the re-parent is real

`research/fixtures/pet-alpha-is-0xff-hook.log`: `141ebc1b2` fired once at the summon with
`rdx=0xff`, and `141ec7880:peek=3c0` read `0xff` on all 357 samples. §9 is eliminated.

`research/fixtures/pet-reparent-user0x3fd0-set-to-1-at-setfield-from-options-hook.log` **[L]**:

```text
142934760     1 hit    while dispatching 0x01A0 SetField, rdx=1, called-from 0x142887193
140f8abc0  6000 hits   [user+0x3fd0] = 0 on the first 39, then 1 on 5961
140f80830  5286 hits   [user+0x3fd8] = 0x346e8660 on every ladder call - a live object
```

## 10. Where the 1 comes from

The call site, in the SetField handler's neighbourhood (`FUN_142886870`, 9966 bytes):

```text
142887178  mov  rcx, [rip+0x1241621]     ; -> 0x143ac87a0 = DAT_143ac87a0
14288717f  test rcx, rcx
142887182  je   142887193
142887184  mov  rax, [r14]
142887187  mov  edx, [rcx + 0x70]         ; the OPTION
14288718a  mov  rcx, r14                  ; the user
14288718d  call [rax + 0x178]             ; -> FUN_142934760(user, option)
```

`DAT_143ac87a0` is the **same options object** whose `+0x58` supplied the pet alpha percentage
(100). `+0x70` is 1 on this machine. So at SetField the client applies one of its own
options, `user+0x3fd0` becomes 1, and from then on `FUN_141eca710` attaches the pet's sprite
to the object at `user+0x3fd8` instead of taking the normal arm. **It is not a byte we sent.**

What the option means is not established - its writer is a generic loader that a displacement
grep cannot see (the object is heap-allocated behind a pointer, so `+0x70` writes are
indistinguishable from every other struct's). `-PetParentOff` tests causation first, with the
probe's `rdx=0` rewrite on the setter - a client patch, and labelled as one in both copies of
the plan. If the Husky draws under it, the remaining work is naming the option and choosing
between fixing the setting, a launcher-side default, and a session patch of the kind the
launcher already carries.



---

# `-PetParentOff`, `-PetLoad`, and the Character Info screenshot

**`-PetParentOff`** (`research/fixtures/pet-reparent-forced-off-still-invisible-hook.log`): the
rewrite took - the setter fired once with its original `rdx=1` and stored 0, `user+0x3fd0`
read 0 on all 3555 samples - and the Husky stayed invisible. §10's re-parent theory is dead.

**`-PetLoad`** (`research/fixtures/pet-frames-loaded-for-six-actions-no-fallback-hook.log`) **[L]**:

```text
140cd8da0     6 hits   the lazy frame loader, all from 0x141ec88e3 inside FUN_141ec87b0,
                       rdx = the template (deref 0x004c4b46 = 5000006), r8 = the action:
                       5 at the summon, then 1, 2, 0, 8, 3 - each action loaded once
141ec86c0     0 hits   THE FALLBACK NEVER RAN - every list was non-empty after its load
141ebdf10   116 hits   all from 0x141ebfec7, the periodic updater, not the fallback path
```

**The pet has frames**, for six actions. And the owner's screenshot of the same session settles it
from the other side: the **Character Info window draws the Husky's body** - the `stand0`
animation, not the icon. The template, the canvas archive and the animation machinery all
work in this client, for this pet, in this session.

## 11. Two corrections

* **§8 over-claimed.** `FUN_141b054f0` gives the name tag its **own** layer
  (`plVar16[i+9]`) and positions it *relative to* the pet's layer via `vtbl[0x238]`; it does
  not put the tag inside the pet's layer. So the tag proves the pet's layer has a valid
  position, not that the pet's layer is itself composited.
* **§2 mis-named `FUN_141ecaa40`.** A reset / set-from-VARIANT / insert-into-layer triple whose
  second arm takes the *user's* object, and which the name tag also uses to hang itself off the
  pet, is an **`IWzVector2D` position**, not a canvas. The frames go through `FUN_141ec87b0`
  and `FUN_140cd8da0`, and `-PetLoad` shows they arrive.

## 12. Where it stands

Measured or forced, each on its own capture: alive; sync says VISIBLE; enable flag never
zeroed; layer opaque; alpha `0xff`; attachment fine; re-parent forced off; frames loaded for
six actions; canvases proven by the info panel. The field layer still contributes nothing.

The one thing found and not yet run down: `FUN_141eca710` computes a proper field z
(`(layer*3000 - y)*10 - 0x3fff8ada`, the same base every field object uses at
`0x140d0ce7d`) and hands it to `FUN_141ecaa40` as `edx` - **which never reads it**
(`0x141ecaa40..0x141ecac02` touches `rdx` only as scratch). The layer was created by
`DAT_143add050->vtbl[0x168](0,0,0,0,0, ...)`. Whether z is applied anywhere else - `FUN_141ec87b0`
calls `layer->vtbl[0x320](8, v)` and `vtbl[0x300](2)` on it - is the open question, and it is a
vtable-slot question that needs the Gr2D interface layout rather than another launch.



---

# `put_z` is `vtbl+0x198`, and the pet's land arm never calls it

Found by following the field-object z constant rather than the pet: `0x140d0ce7d` computes
`z = (layer*3000 - y)*10 - 0x3fff8ada` for a generic field object and the wrapper at
`0x140d0cce8` applies it as `layer->vtbl[0x198](layer, z)` **[L]**. So `+0x198` is `put_z`.

Every `vtbl[0x198]` call in the pet code **[L]**:

| where | value | arm |
|---|---|---|
| `FUN_141ec2690` @ `141ec26f3` | `rsi+1` | the `pet[0x7d]` arm - nested under another object |
| `FUN_141ec2690` @ `141ec2885` | `1` | the `pet[0x7b]` arm - nested |
| `FUN_141ec22f0` @ `141ec23d6` | `param_4` (3) | the **flying** arm, stance 1 |
| `FUN_141b054f0` @ `141b084fc` | `0x2325` | the **name tag's own** layer, for `0x3eb` |

The free-standing land arm - `pet[0x7d]==0`, `pet[0x7b]==0`, `pet[0x77]!=0`, stance 0, ours -
is not in that table. It calls `FUN_141ecaa40(pet, z, &vector)` with the z `FUN_141eca710`
computed, and `FUN_141ecaa40` never reads `edx` (`rcx->rdi`, `r8->rbx`, `rdx` unspilled and
untouched until reused as scratch at `141ecaac4`). The layer was created by
`DAT_143add050->vtbl[0x168](0,0,0,0,0, ...)`. **In our arm the pet's layer keeps z = 0 for its
whole life**, while the tag hanging off it sits at `0x2325` and draws.

Whether z = 0 is what hides it is not established - it is [I] - and the flying arm is the
one-byte way to find out without a client patch: `moveAction` 30 decodes to 8, `FUN_141ec7e90`
returns 1, `SetStance(1)` takes `FUN_141ec22f0`, which calls `put_z` and has its own insert
path, and the Husky has a two-frame `fly`. `--pet-move-action` / `-PetMoveAction 30` is that
lever, one byte on the wire (`the_pet_move_action_lever_changes_exactly_one_byte_of_the_summon`).
It is a localising experiment: a Husky should not fly, and if it appears the real question
becomes what sets z on a real server - most likely a packet this one never sends.



---

# `-PetMoveAction 30`: the flying arm draws nothing either

`research/fixtures/pet-moveaction-30-flying-arm-still-invisible-world-ch0.log`: byte 33 of the
`0x0277` body is 30, 90 pet-move reports follow, and the Husky is still not on screen. So the
missing `put_z` in the land arm is not it - **both arms share the fault**.

## 13. The working presenter, found

`FUN_140cd8da0`, the frame loader, has seven callers and one of them is **`FUN_1414bc3f0`** -
the Character Info window's pet drawer, the code behind the body in the owner's screenshot. Same
loader, same template (`FUN_141ed3540(itemId)`), same `CreateLayer` (`DAT_143add050->vtbl[0x168]`
with five zero ints - 231 functions do exactly that, mobs included), same `InsertCanvas`
(`vtbl[0x258](canvas, VARIANT delay, ...)` per frame), same `0x1e0 / 0x310 / 0x318 / 0x330`
afterwards. **It draws. `CPet` does not.**

What the two do differently to the layer, as vtable offsets **[L]**:

| | panel `FUN_1414bc3f0` | field `CPet` (`Init` + `FUN_141ec87b0`) |
|---|---|---|
| before inserting | `vtbl[0x200](-1)` | `vtbl[0x268](VARIANT(VT_I4, -2), &out)` |
| position | `vtbl[0x238](&vector)` from `FUN_142bf6010(wnd)` - window-relative | `vtbl[0x238](VARIANT(pet+0x3b8))` - field-relative |
| z | `vtbl[0x198](1)` | nothing in the land arm; `(3)` in the flying arm |
| after inserting | - | `vtbl[0x300](2)`, `vtbl[0x320](8, v)`, `vtbl[0x280](0x20, BSTR, v)` |

Those slots cannot be named from the client side; the engine is `Gr2D_DX11.dll` (the three
IIDs the client raises on - layer `cec7c86d-…`, vector `db126d03-…`, root `33ea76e5-…` - are
all in it, and it carries no RTTI or type library for its own classes). It is being imported
into the Ghidra project so the implementations behind `0x268`, `0x300`, `0x320` and `0x280`
can be read directly. That is the next step, and it is not a launch.

