# Custom Mix Dye and Custom Colorblend: how this client mixes hair and eye colours

**2026-10-03.** The owner asked how mix dye hair colour and colorblend eye colour work in this
client, so they could be offered the way the Signature colour coupons already are.

Markers: **[L]** read off this client's listing or decompile, **[D]** derived from two or more [L]
facts, **[I]** inferred.

Dumps, all new today:

| file | what |
|---|---|
| `msexe-mix-avatar-handlers.c` | the avatar-family `0x055B` handlers and the dialog-kind switch `FUN_142a58880` |
| `msexe-mix-ask-handler-14127e090.asm.txt` | the mix ask handler's listing |
| `msexe-mix-value-encoding.c` | the dialog's value, the coupon typing, the compose/compare dispatch |
| `msexe-mix-id-arithmetic.c`, `msexe-mix-id-compose.c` | the hair and face id arithmetic |
| `msexe-mix-current-look.c` | where the box gets the player's current hair and face |
| `msexe-mix-dialog-types-42-46.c` | types `0x42..0x46`, ruled out (below) |

---

## 1. The answer

**A mixed look is NOT a new field. It is a bigger id in the same `hair` and `face` slots.**
**[L]**:

```text
mixed hair = (style + baseColour) * 1000 + mixColour * 100 + percent      FUN_14041a8d0
mixed face = (face with eye colour base) * 1000 + mixColour * 100 + percent FUN_14041a820
```

Every look helper first does `if (id > 9999999) id /= 1000`. That covers the colour read
`FUN_14041a0f0`, the colour swap `FUN_14041a7e0`, the eye colour read `FUN_14041a0a0` and the
eye swap `FUN_14041a780`, so a plain id passes through every path unchanged. `FUN_14041a520`
(hair) and `FUN_14041a480` (face) split a mixed id back into `(base, mix, percent)`. The hair
composer refuses a colour of 8 or more (`param_2 < 8 && param_3 < 8`) and a percent outside
`1..99`. The face composer has no colour bound of its own.

Black and Blue at 30 on Frieren Hair 42540 is `42540530`. Eye colours 2 and 7 at 60 on face 22035
is `22235760`. The avatar look, `SetField`, `0x0224` and the `0x007C` HAIR/FACE bit already
carry a `u32`. **[D]**: no packet layout changes.

**The percent is the MIX colour's share** (settled the same day, §7): the window labels
`/AddProb` with it beside `/AddColor`, and `/BaseProb` with `100 - percent` beside `/BaseColor`
(`FUN_142a93ac0`). `FUN_14041a2a0` treats `(a, b, p)` and `(b, a, 100 - p)` as the same look.

## 2. The box: `0x055B` type `0x2a`

The 71-entry type table at `0x141f6f9f4` sends types `0x29`, `0x2a` and `0x40` to one handler,
**`FUN_14127e090`** (stub `141f6f63b`). The body is four reads, and the listing and the
decompile agree:

```text
14127e0f6  u32  coupon
14127e105  str  text
14127e117  raw1 mode     0 = the player's own look; 'd' = an android's
14127e128  u32  the STARTING percent - list element 0, read by the setup (§7). 0 breaks it
```

**The coupon picks the dialog** (`FUN_1401a8170` -> `FUN_140417ed0`):

| coupon | cash type | item type | dialog kind | window | reply type |
|---|---|---|---|---|---|
| `5151200` Custom Mix Dye | `0x59` | `0x18` MixHairColor | `0x17` | `UtilDlgEx_MixHair` (`FUN_142a8d5e0`) | **`0x2a`** |
| `5152300` Custom Colorblend | `0x56` | `0xe` MixColorLens | `0x19` | `UtilDlgEx_MixLens` (`FUN_142a8d5e0`) | **`0x40`** |

The item-type names come from `FUN_1401a9530`'s string switch: `0xe` "MixColorLens" and `0x18`
"MixHairColor"; `0xd` is "ColorLens" and `0x17` is "HairColor". The reply type is the client's
choice, made from the coupon and not from the type sent.

**Mode 0 means the player's own look.** `FUN_142a8a3e0` -> `FUN_142dcd020` -> `FUN_1401a6ee0`
reads the current hair (`+0x23`) or face (`+0x1f`) from the local user, so the server sends no
look.

## 3. The answer: `0x00F3`

Written at `14127e675`:

```text
u32 0, u8 type (0x2a hair / 0x40 lens), u8 ok
  when ok: raw1 mode, u8 0, u32 (0 unless the head said 4), u32 value
```

`value` is the dialog's own encoding, `FUN_14041a3d0`: `(base * 10 + mix) * 1000 + percent`,
valid only with both colours under 10 and the percent in `1..99` (`FUN_14041a270`). The client
composes the new id with the current look (`FUN_1401a8660`). If it equals the current look by
`FUN_1401a8500` (same id, or the swapped twin), it shows its own warning (string `0x479` for
hair, `0x475` for lens) and answers `ok = 0`.

## 4. Ruled out: types `0x42..0x46`

These were the five undecoded types in `beauty-2026-09-09.md`. They take `u8, u32` and similar,
then look the `u32` up in a template pool (`FUN_141e768f0`). That is a different dialog family
(`FUN_1410dcc60`) with no colour, no avatar and no mix string. Nothing in them is the mix box.

## 5. What the server does (2026-10-03)

* **The two assistants each get a fourth menu line, `#L3`.** Brittany and Andre take the Mix Dye
  coupon; Dr. Feeble and Riza take the Colorblend coupon. The line sends `net::script::npc_mix`.
* **The answer is a claim.** The server checks it against its own record before anything else
  happens:
  * the reply type must match the desk;
  * `Blend::from_reply` must decode the value;
  * the style comes from the stored hair or face;
  * both colours must have art (`Config::hair_exists` / `face_exists`; hair colours under 8, eye
    colours under 9);
  * the look must not already be worn (`salon::same_look`, the client's rule).

  Only then is the look written, the coupon spent, one HAIR or FACE bit sent and the field told.
* **Every existing look helper reads the plain id under a mixed one.** That covers `base_of`,
  `colour_name`, `face_style_of`, `eye_colour_variants` and the notices. A style change keeps
  the mix when the new style draws both colours, and drops to the first colour when it does not.
  A Signature or Mystery dye gives a plain colour, which clears the mix.

## 6. Still open (see §7 for the first run)

* **[I], the one a client run settles: whether this client DRAWS a mixed id.** The dialogs, the
  arithmetic and the strings are all here, and the composer feeds the result straight back as
  the look. But no capture has ever carried an id above 9 999 999, and the renderer's
  `Character/Hair/%08d.img` lookup was not traced past `/1000`. A blank head, or a crash on
  the HAIR bit, would be that.
* **The coupons cannot be bought.** Neither has a `Commodity.img` row
  (`beauty-2026-09-09.md` §2.2). `set_cash_shop` carries a commodity delta that might add one;
  that is untested. For now a GM `!item` is the only source.
* The `_New` and `_KR` windows (`FUN_142a8e830`, `FUN_142a8de60`) are dialog kinds `0x18`,
  `0x1a` and `0x1b`. They are reached from types `0x2b` and `0x3b`, which this server does not
  send.

## 7. 2026-10-03, the first run: the window opened and no colour could be picked

The owner, with a screenshot: the MixHair window opened on the character at Brittany, *"but I cannot
select any of the colors for preview"*. That was this file's own mistake. §2 said the box's last
`u32` was "stored as list element 0; nothing reads it back". The handler was the only function
read. **The window's setup reads it.** In `FUN_142a91f30` (`research/msexe-mix-hair-window.c`),
case `0x17`/`0x19`:

```c
uVar8 = *puVar10;                        // list element 0: the u32 we sent
if (param_3 - 0x1cU < 2) uVar8 = 0x32;   // the client's own kinds 0x1c/0x1d force 50
FUN_14041a250(state, rnd & 7, (rnd & 7) + 1 & 7, uVar8);   // base, mix, PERCENT
```

`FUN_14041a250` stores the three bytes as they are. Every part of the window is then gated on
`FUN_14041a270` (`research/msexe-mix-window-state.c`):

* `FUN_142a91c70` marks both palettes `-1`, nothing selected, when the state is invalid;
* `FUN_142a93870` builds the ratio slider only when it is valid;
* `FUN_142a93ac0` draws the two percent labels only when it is valid.

We sent 0, which is outside `1..99`, so the window opened dead. **The fix:** send 50, the value
the client forces for its other two kinds (`net::script::MIX_START_PERCENT`).

The habit this breaks again is `CLAUDE.md`'s *"a table row written from a quick read is a
claim"*. "Nothing reads it back" was checked against the handler alone. The value went into a
list, and the list went to another function that was never read.

**The same zero made Cancel draw a warning.** In the owner's second screenshot, Cancel brought up
*"This cannot be used. Hair with the same color is already equipped."* That is the client's
string `0x479`, and §3 already had the cause without seeing it. After the window closes,
`FUN_14127e090` reads the window's value (`FUN_142a8a7d0`) and composes it with the current hair
(`FUN_1401a8660`). It then runs the same-look check (`FUN_1401a8500`) **before** looking at
which button was pressed; the confirm flag `uVar13` only matters when the reply is written.

With an invalid state the value is 0. `FUN_14041a8d0` refuses a percent of 0 and returns the hair
unchanged, so the check found a match and warned on every Cancel. With a valid start, Cancel
composes the random starting mix, and that is not the worn look.

One coincidence was left: a player already wearing a 50% mix whose random starting colours match
it. `salon::mix_start_percent` opens such a player at 51 instead. A 51% start cannot match a 50%
mix either way round (its swapped twin is 49%).

A Confirm on the look already worn still draws that warning, and should: it is the client
refusing to spend a coupon on nothing.

