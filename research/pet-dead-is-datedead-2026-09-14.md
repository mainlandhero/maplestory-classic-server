# The pet was dead because `dateDead` was the never-expires sentinel - 2026-09-14

The owner, with the Husky's tooltip: *"The water of life has dried up."* And, the round before,
on screen: *"Cannot move because the magic duration has ended. Use the Water of Life to
revive them."*

Labels are the project's: **[L]** read off this client · **[D]** derived · **[I]** inferred ·
**[R]** the reference server. **Everything below is [L].** No client run was spent on it.

---

## Summary

`FUN_1402cf680(item, 0)` is the whole deadness question, and it is a three-way branch on the
pet's own WZ image:

| the image says | the verdict |
|---|---|
| `limitedLife > 0` (`FUN_14038a300`, key `"limitedLife"`, default `0`) | dead iff `remainLife <= 0` |
| `life == 0` (`FUN_14038a380`, key `"life"`, default `1`) | **alive, unconditionally** |
| otherwise | dead iff `dateDead >= 150842304000000000` |

`Item/Pet/5000006.img/info` is `{hungry 2, cash 1, life 7, permanent 1, pickupItem 1, ...}` -
no `limitedLife`, and `life` is not zero - so **the Husky takes the third row**, and the third
row's constant is `ITEM_NEVER_EXPIRES` byte for byte. `net::bag` was sending exactly that as
`dateDead`. The client was being told, in its own vocabulary, that the pet is a doll.

`net::bag::PET_DATE_DEAD` is now `150_211_584_000_000_000` - 2077-01-01, fifty years clear of
both ends of `now < dateDead < ITEM_NEVER_EXPIRES`.

---

## 1. The three rows, as bytes

```text
0001402cf694  mov  rbx, [rip+0x37d8c8d]          ; the item-info singleton
0001402cf69e  add  rcx, 0x20                     ; item+0x20, the obfuscated itemId
0001402cf6a5  call 0x1401b0340                   ;   -> eax = itemId
0001402cf6af  call 0x14038a300                   ; "limitedLife" > 0 ?
0001402cf6b6  je   0x1402cf6d3
0001402cf6b8  mov  edx, [rdi+0x92]               ; ---- row 1: limited-life pets
0001402cf6be  lea  rcx, [rdi+0x8a]               ;      the remainLife triple
0001402cf6c5  call 0x1401ba9d0
0001402cf6ce  setle cl                           ;      DEAD iff remainLife <= 0
0001402cf6e8  call 0x14038a380                   ; "life" == 0 ?
0001402cf6ef  je   0x1402cf6f5
0001402cf6f1  xor  ecx, ecx                      ; ---- row 2: ALIVE, nothing else read
0001402cf718  mov  rax, [rip+0x3805f29]          ; ---- row 3: CompareFileTime
0001402cf71f  lea  rdx, [rip+0x2fae662]          ;      -> 0x14327dd88
0001402cf726  lea  rcx, [rdi+0x82]               ;      the pet's dateDead
0001402cf72d  call rax
0001402cf733  setns cl                           ;      DEAD iff dateDead >= that constant
```

and the constant:

```text
0x14327dd88:  00 80 05 bb 46 e6 17 02   = 150842304000000000 = 1601-01-01 + 2079 years
              ^ ITEM_NEVER_EXPIRES, the same eight bytes this server hands every item as
                "no expiry", and the same eight bytes it was handing every pet as dateDead
```

The two WZ readers are ordinary property lookups and their key strings are in `.rdata`:
`0x1432ab4c0` is `"limitedLife"`, `0x1432ab490` is `"life"`. `FUN_14038a300` ends
`setg` (dead-relevant iff `limitedLife > 0`, default `0`); `FUN_14038a380` ends `sete`
(returns 1 iff `life == 0`, default `1`).

## 2. Where the verdict is consumed

* **The tooltip.** `FUN_14266f2d0`, line 281 of `research/msexe-pet-tooltip.c`:
  `if ((param_5 == 0) && (FUN_1402cf680(param_4,0) != 0)) { ... PTR_s_descD ... }` - so a dead
  pet's description comes from the WZ's `descD` instead of `desc`. For 5000006 that is
  *"This was once a cute little Husky, but it turned back into a doll when its Water of Life
  dried up."* - the exact words on the owner's screenshot, out of
  `String_000.wz/Pet.img/5000006/descD`.
* **Moving it.** `FUN_142d4ced0` at `0x142d4d21e` casts the slot with `FUN_140192f80`, calls
  the pet vtable's `+0x1b0`, and raises string `0x1A9` - *"Cannot move because the magic
  duration has ended"* - when it comes back false. That vtable is `0x14327e910` (installed by
  `FUN_1402f8b40`; `+0x358` is the pet decoder `FUN_140304550`, which is how it was found).

## 3. Why two runs missed it, and what that eliminates

`info/life` had been zeroed for every pet by our own installer - the change
`cabb8e2` reverted. **`life == 0` is row 2, which returns ALIVE without ever reading
`dateDead`.** So a bad `dateDead` had been in every pet body since the day the body was
written and could not be seen. Restoring `life: 7` - correct in itself - moved the client onto
row 3 and made the pre-existing bug audible. The order of the owner's two screenshots is exactly
this: silence, then *"the magic duration has ended"*, then `descD`.

`PET_REMAIN_LIFE = 1_000_000_000` (commit `6b81e49`) was pre-registered as a one-variable
candidate. The run came back unchanged, and the honest reading is **not** that it was
disproved: row 1 needs `limitedLife > 0`, this image has no `limitedLife`, so the field was
never consulted. It is kept non-zero because a pet image that *does* carry `limitedLife` would
otherwise be dead the instant it is summoned. **An inert variable is not a tested one** - the
same trap `CLAUDE.md` records for the timestamp filter and the `0x00D5` burst.

## 4. Round 1 confirmed, and it was only half the problem

The owner, with the tooltip open: **"Water of Life Dries Up: 1/1/2077 00:00 UTC"** and the normal
description back. The item is alive. The pet summons. **It still does not draw and it still
does not pick up.** The pre-registered reading for that outcome was "TWO bugs, not one", and
that is what this is.

`research/fixtures/pet-alive-datedead-2077-summons-and-moves-but-does-not-draw-world-ch0.log`
is that run.

## 5. What the run rules out, without spending another launch

| ruled out | how |
|---|---|
| the packet | all fourteen fields of `0x0277` decode to what we meant, 50 bytes exactly: charId 215, petIdx 0, activated 1, init 1, itemId 5000006, "Husky", the serial, x 77, y 65, moveAction 0, foothold 154, hue -1, itemId again, wonderGrade 0, giantRate 0, nameTag 0, chatBalloon 0 |
| the position | the pet is at (77, 65) and the owner's last `0x00D9` before the summon ends at x = 77. It is at their feet, not off-screen |
| the object | `0x0202` arrives **166 times** after the summon. The pet exists, ticks and walks |
| the art | `Item/Pet/5000006.img` still has all 22 action nodes and every frame keeps its `_outlink` |
| the archive rewrite | **the inventory icon is the control.** It is the same shape of node - a 1x1 placeholder plus `_outlink` into `Item/Pet/_Canvas` - and it draws in the owner's screenshot. So outlink resolution survives `backport_install.py`'s rewrite for this exact image |
| a client fault | no `CLIENT FAULT` in the hook log, and the one `0x008F` ELog predates the summon |
| our WZ rewrite, conclusively | `Pet_000.wz` and the **untouched original** `Pet_000.wz.bak` are identical in shape at `info/icon`, `stand0/0` and `move/0` - all three 1x1 placeholders carrying an `_outlink` - and `_Canvas_000.wz` (Aug 11, never rewritten) holds the real 41x37 and 47x40 bitmaps. `backport_install.py` touched no art |
| culling, and being off-screen | **the "Husky" name tag is drawn**, beside the owner, in the 2026-09-14 round-2 screenshot |

### The name tag is drawn, and what that does *not* mean

It would be easy to read the tag as proof that the visibility sync ran, because
`FUN_141ecaf00` - which builds it, out of `pet+0x138` and a length check against 12
characters - is the last call in the show path. **It is not.** `tools/callers.py` gives
`FUN_141ecaf00` six call sites, and one of them is `CPet::Init` itself
(`0x141eb9760`, at `0x141ebc162`). The tag is built at construction whether or not the pet is
ever shown.

What the tag *does* settle is that the pet is on screen and un-culled, and that the failure is
specific to the sprite. The show path toggles three different sub-objects -
`[pet+0x3c8]->vtbl[0x2b8]`, `FUN_14159b0a0(pet+0x40, v)` and `pet->vtbl[0x18](v)` - so "the tag
draws and the sprite does not" is consistent with either a transition that never ran or one
that ran on only part of the object. `-PetSync` separates those; the tag does not.

## 6. The correction that makes the next run worth spending

`research/pet-not-drawn-2026-09-13.md` reads `FUN_141ecde00` as a verdict, and the `-PetGates`
and `-PetFlags` runs were designed against that reading. **It is a sync**, and the tail says so:

```text
141ecde15  xor  r14d, r14d          ; r14 = 0
141ecde1f  mov  edi, r14d           ; edi = DESIRED = hidden
141ecde27  lea  ebp, [r14+1]        ; ebp = 1
   ... eleven gates; EVERY failure jumps to 141ecdf15 with edi still 0 ...
141ecdf0b  call 142cc1e40
141ecdf12  cmove edi, ebp           ; gate 11 returns 0 -> DESIRED = 1 = VISIBLE
141ecdf15  ebp = the renderable's CURRENT state, or 0 when pet+0x3c8 is null
141ecdf56  cmp  ebp, edi / je       ; EQUAL -> return, touch nothing
141ecdf8f  call 14159b0a0(pet+0x40, edi)   ; only on a change, and rdx IS the verdict
```

So "`14159b0a0` was never called" - the run-2 result the whole hunt rests on - does **not** mean
the pet was refused. It means *desired equalled current*, and that has two opposite readings:

* every gate passed, desired 1, and the client **already had the pet visible**, or
* a gate failed, desired 0, and the pet was already hidden.

The 2026-09-13 note picks the second and never states the first. Both fit every byte of that
capture. And the `-PetFlags` design inherits a second flaw: it watched the two **setters** of
`+0x24ac` / `+0x24b0` and read "no setter fired" as "the fields are zero". That is the
`mob+0x42c` blind spot again - a field can be written through a pointer no setter-watch sees.

`-PetSync` replaces both. Four watches, one capture, and it can come back false:

```text
141ecde00:hits=6000                the ladder runs for the pet at all
142cc1e49:peek=24ac:hits=6000      gate 11 READ, not inferred: reaching the second compare
                                   proves +0x24b0 == 0, and peek prints +0x24ac. No call has
                                   happened at that address, so called-from still reads the
                                   ladder's 0x141ecdf10
14159b0a0:hits=6000                the sync changed something, and rdx says to what
140304100:hits=200                 positive control
```

The outcome that would close the ladder for good is **`142cc1e49` with peek 0 and no
`14159b0a0`**: desired 1, current 1, the client already believes the pet is visible, and the
failure is in drawing rather than in visibility. The outcome that reopens it is `142cc1e49`
absent, or its peek non-zero.


---

# The `-PetSync` run: the client already thinks the pet is visible

One capture, 2026-09-14 16:51:45, kept as
`research/fixtures/pet-sync-says-visible-2964-times-and-nothing-draws-hook.log` (and the
channel side as `pet-sync-run-world-ch0.log`). Every number below is from that one file.

```text
0x140304100    36 hits      POSITIVE CONTROL - the hook armed and the log is evidence
0x141ecde00  4164 hits      the ladder ran for the pet. First hit at the summon
                            (0x0277 at 20:51:55.099); 3322 more from the periodic
                            updater 0x141ec1ce8, ~30 a second, as designed
0x142cc1e49  6000 hits      of which 2964 with called-from=0x141ecdf10, the ladder.
                            [rcx+0x24ac] = 0x00000000 on ALL 2964, and reaching that
                            instruction at all proves [rcx+0x24b0] was 0 too
0x14159b0a0     2 hits      both from 0x141e4c056, both at 16:51:53 - two seconds
                            BEFORE the summon, during opcode 0x007C. NEVER from the
                            ladder's 0x141ecdf8f
```

Read against the tail in §6:

* gate 11 returns 0 on every evaluation, so `cmove edi, ebp` fires and **desired = 1 = visible**;
* the sync never transitions, so **current == desired**;
* therefore **current = 1. The client has the pet marked visible, 2964 times over, and draws
  nothing.**

**The visibility ladder is innocent and this closes it.** `research/pet-not-drawn-2026-09-13.md`
spent three runs on a mechanism that was answering "yes" the whole time; what made that
invisible was reading a sync as a verdict, so that "no transition" looked like "refused".

## What that leaves, and the control that killed the last cheap theory

The pet is visible, positioned at the owner's feet, iterated 4164 times, its name tag is drawn, and
its sprite is not. The obvious remaining theory was that the sprite resolves to the **1x1
placeholder** - `Item/Pet/5000006.img/stand0/0` is a 1x1 canvas carrying an `_outlink` into
`Item/Pet/_Canvas`, and a failed resolve would draw one transparent pixel, which looks exactly
like this.

**It is dead, and the control is on the owner's screen.** `Mob/Mob_000.wz/0000001.img/move/0` is the
*same shape* - 1x1, 10 bytes, `_outlink: Mob/_Canvas/0000001.img/move/0` - and the orange
mushrooms and snails in the screenshot are drawing. Outlink resolution works for field
animations in this client, in this session.

So the failure is between "the layer is visible" and "the layer has something to draw".
`[pet+0x3c8]` is reached by `QueryInterface` (`vtbl+0x168`) and answered `get_visible` through
`vtbl+0x2b0`; HRESULT returns and a vtable that large make it one of the client's `IWzGr2D*`
layer interfaces, which is also the object an animation's canvases get inserted into.

**The next step is not a launch.** It is a Ghidra decompile of `CPet::Init` (`FUN_141eb9760`,
11 005 bytes) and the two `FUN_141ed3540` template loads it requires, to find where a pet's
canvases are inserted into that layer and what selects the action - the one field we send that
could choose an animation is `moveAction`, and we send `0`. Nothing about that is established
yet, and it is deliberately written here as the question rather than as a candidate.
