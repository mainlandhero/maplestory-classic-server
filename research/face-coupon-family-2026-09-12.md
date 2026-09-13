# The face coupons opened nothing: the Beauty dialog's id ranges - 2026-09-12

The owner: *"The face coupons from the backported collaboration items still does not work."*

Tags: **[L]** a log line, a WZ leaf or a listing; **[I]** inferred.

## 1. No packet

The hair coupons (2543xxx) work: their double-click opens the client's Beauty Coupon dialog
and CONFIRM sends `0x0165` (`crates/net/src/beautycoupon.rs`). For a face coupon (2897xxx)
world.log shows nothing at all, in this run and every archived one. **[L]** Client-side
refusal, so the item's id and data are the suspects - as with the box and the hats.

## 2. The dialog's opener accepts seven ranges

`FUN_142dc8100` builds the dialog. Its three callers (`tools/callers.py`): `0x142ccc3a0`
(an inbound handler), `0x142dcb430` (a 24-way switch - another route in) and
**`0x141784fa0`**, the Use-tab item path, which gates it like this **[L]**:

```text
141785d90  lea eax,[rbx-0x2c24c8] ; cmp eax,0x3e8 ; jb open     2900168..2901167
141785d9d  lea eax,[rbx-0x2c3080] ; cmp eax,0x3e8 ; jb open     2895000..2895999
141785daa  lea eax,[rbx-0x2c1910] ; cmp eax,0x3e8 ; jb open     2889000..2889999
141785db7  lea eax,[rbx-0x2c1cf8] ; cmp eax,0x3e8 ; jb open     2890000..2890999
141785dc4  lea eax,[rbx-0x2c28b0] ; cmp eax,0x3e8 ; jb open     2893000..2893999
141785dd1  lea eax,[rbx-0x2c2c98] ; cmp eax,0x3e8 ; jb open     2894000..2894999
141785dde  lea eax,[rbx-0x26c1e0] ; cmp eax,0x2710 ; jae skip   2540000..2549999
141785dfe  call FUN_142dc8100                                    open the dialog
```

`rbx` is the item id. 2897xxx is in none of the seven. The modern client's own String.wz
names the families (`Consume.img`, read with `wz-dump cat`): 2540..2545 are hair coupons,
**2890 and 2891 are "Face Coupon" (43 of 52 and 45 of 54 entries)**, 2893 skin coupons,
2894/2895 android face coupons, 2892 android parts. So the classic client opens the dialog for
exactly the hair and face families the modern client used at the time this client was built,
and Nexon later moved these particular face coupons to 2897. **[L]** for the ranges and names;
which of the seven the dialog treats as *face* is **[I]** from the names - 2890 is the one.

## 3. Installed

`tools/backport_install.py`: `FACE_COUPON_RENAMES` = 2897007..2897014 -> 2890907..2890914
(the modern client uses 2890000..2890054, so the stretch is free), folded into a general
`RENAMES` with the box. Step 3 merges each renamed node under its new key (same `0289.img`,
same canvas nodes - the icon outlinks still name `02897xxx` under the 0289 canvas, which is
merged untouched); the strings step carries name and desc to the new key. Server side:
`world::cosmetics` (coupon -> face id), `world::signaturestyle` (what each set hands out),
`store::ITEM_ID_RENAMES` (coupons already in a bag are renumbered on the next open). Built
from the pristine bases, installed with the client closed, read back: `0289.img` holds
`02890907..02890914` and nothing under 2897; items.txt names them. 1389 store+world tests.

## 4. Not established

* That the dialog previews a *face* for a 2890xxx id rather than keying on something else.
  Plan step TO(j) names the outcomes.
* Nothing on a screen yet.
