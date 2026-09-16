# Item buffs and their icons: `-itemId` as the reason, and CTS 163 for the EXP coupon

2026-09-16. The owner: *"I just noticed that the EXP coupon effects are not applying the appropriate
buff icon on the top right of player's screen. Please make sure 2x and 3x coupons have the
proper buff durations applied to the player in addition to their effects."* Then: *"Please also
make sure that Magic Potions and other similar potions are applying the buff icons as well."*

## What was being sent

`Session::buff_from_item` (`crates/world/src/session/consume.rs`):

| item | `0x007D` sent | reason field |
|---|---|---|
| Magic Potion `2002001` (`mad 10`, 600 s) | yes, bit 85 | `2002001` **positive** |
| GM's Blessings `2023000/1` | yes, map-wide | positive |
| 2x / 3x EXP Coupon `2450000/1` | **none** | - |

The coupon multiplies server-side (`with_exp_coupon`, since 2026-09-09) and was never given a
CTS bit, so there was nothing on screen to count down. The potions were on screen as a stat
(M.Att rose) with no icon.

## The reason's sign **[D]**

A positive reason is a skill id: the icon and tooltip come from `Skill.wz`. `2002001` is not a
skill. The modern reference source (`Char.java` `o.rOption = -buffData.getItemID()`,
`ItemBuffs.java`) sends every item buff with the id **negated**, and that is the convention in
every client version anyone has documented. It is labelled [D] because **no instruction in
this build has been read testing the sign** - the search for the icon builder found the skill
icon path `Skill/%03d.img/skill/%07d/icon` referenced by two functions that are keyed on
specific skill ids (`FUN_14210df20`, `FUN_142109b80`), not the general stat-change window, and
the window's singleton (`0x143aa8518`, called through vtable slot `+0x80` from the `0x007D`
handler at `0x142d566c5`) resolves at runtime. `xref.py` finds no code reference to
`UI/BuffIcon.img/IconBase/0`, which is the tool's known blind spot for inline copies, not
evidence the string is unused. One launch settles it; plan step 10 has the outcome table.

## Bit 163 **[L] for the name, [D] for the semantics**

The client's own CTS name table (`research/first-job-buffs.md`, Appendix A, re-derived with
controls) calls index **163 `ExpBuffRate`**. Its decoder block:

```text
140a22478  mov  edx, 0xa3            ; bit 163
140a22485  call 0x1402bf6d0          ; is it set
140a224a5  call 0x140a10800          ; wide-value stat?   -> u32 value
140a224d7  call 0x1406e8b80          ;                   else u16 value (cwde)
140a224f4  call 0x1406e8c20          ; u32 reason
140a22516  call 0x1406e8c20          ; u32 duration, added to the base time
```

Same shape as bits 83..92 and 97, so `temporary_stat_set`'s 10-byte entry fits it. The value
sent is the item's `expBuff` percent (`200`, `300`), which is what the modern reference sends
on this bit. The client's *use* of the value is unmeasured and does not matter here: the
multiplier is the server's.

## What changed

* `net::buff::CTS_EXP_BUFF_RATE = 163`, `net::buff::item_reason(id) = id.wrapping_neg()`.
* `buff_from_item` builds every item stat with `item_reason`, and the coupon pushes a bit-163
  entry into the same `0x007D`, tracked in `self.buffs` so `buff_tick`'s `0x007E` clears it at
  the instant `with_exp_coupon` stops.
* Test `an_exp_coupon_and_a_magic_potion_send_their_stat_with_the_item_as_a_negative_reason`.

## If the run says no

If the potion's icon does not appear with a negative reason but the stat still applies, the
client ignores the sign and this build wants something else - the next thing to read is the
stat-change window's constructor (whoever writes `0x143aa8518`) and its slot `+0x80`. If the
potion's icon appears and the coupon's does not, 163 is the wrong bit for the *icon* even
though it is the name-table's `ExpBuffRate`; candidates are `229 PlusExpRate` and the Indie
space, and the multiplier keeps working either way.
