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

## 4. What is still open

The pet being dead explains the refusal to move and the tooltip. It does **not** by itself
explain the earlier measurement in `research/pet-not-drawn-2026-09-13.md`, where all eleven
visibility gates passed and `FUN_141ecde00` was measured wanting to SHOW the pet. Whether a
live pet now draws, and whether it picks up drops, is what the next run is for - and those are
two separate readings, not one.
