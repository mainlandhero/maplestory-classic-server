# Why no star was ever offered Recharge, and what the window prices it from

**2026-10-03.** The owner: *"The clients couldn't have asked to charge because the NPC shop don't
display them as rechargeable."* `STATUS.md` named the open lead: `FUN_141fa77a0`, which fills the
Sell list `shopUI+0x360`, had never been decompiled.

Markers: **[L]** read off this client's listing, **[D]** derived from two or more [L] facts,
**[I]** inferred.

Dumps, all new today:

| file | what |
|---|---|
| `msexe-shop-sell-list-141fa77a0.c` | `FUN_141fa77a0` (the Sell list builder) and `FUN_141fb9cf0` (tab -> list), `+callees` |
| `msexe-shop-sell-list-141fa77a0.asm.txt` | its listing, bounded by `.pdata` (1290 bytes) |
| `msexe-shop-sell-list-141fb9cf0.asm.txt` | the tab lookup's listing (162 bytes) |
| `msexe-shop-row-renderer-141faecc0.asm.txt` | the row renderer `FUN_141faecc0`, whole (10262 bytes) |
| `msexe-shop-recharge-label-141fafc00.asm.txt`, `-141fb0040.asm.txt` | the two windows of it read first |
| `msexe-shop-recharge-button-141fa6360.c` / `.asm.txt` | the button handler that calls the Recharge builder |

---

## 1. The answer

**`row+0x10c` is the full stack the window prices a recharge against, and every star row this
server sent carried `1` there.** **[L]**, all from the listing:

```asm
; FUN_141fa77a0, per bag item; RSI = the new Sell entry, R15 = shopUI
141fa79a0  CALL [RDX+0x98]                ; the bag item's count
141fa79a6  MOV  [RSI+0xc], EAX
141fa79a9  MOV  qword [RSI+0x40], 0       ; recharge price starts at 0.0
141fa79dc  LEA  EAX,[RCX-0x1f95f0] / CMP 0x2710 ; 207xxxx ...
141fa79e9  LEA  EAX,[RCX-0x238d90] / CMP 0x2710 ; ... or 233xxxx, else skip
141fa7a00  MOV  RCX,[R15+0x358]           ; the Recharge list
141fa7a2f  CMP  [RCX+RDI+0x8], EAX        ; first row with the same item id ...
141fa7a33  JZ   0x141fa7a40               ; ... wins (the loop stops at it)
141fa7a40  MOV  EAX,[RCX+RDI+0x10c]       ; that row's +0x10c
141fa7a47  SUB  EAX,[RSI+0xc]             ;   minus the count held
141fa7a52  MULSD XMM0,[RCX+RDI+0x40]      ;   times that row's unit price
141fa7a58  MOVSD [RSI+0x40], XMM0         ; -> the entry's recharge price
```

and the renderer draws Recharge only when that price is **above zero**:

```asm
; FUN_141faecc0, the row renderer, reading shopUI+0x360 (141faed61)
141faed3b  XORPS  XMM6,XMM6               ; XMM6 = 0.0 for the whole function
141faf1e2  MOVSD  XMM0,[RDX+RDI+0x40]     ; a 207/233 entry's recharge price
141faf1e8  COMISD XMM0,XMM6
141faf1ec  JBE    0x141faf69d             ; <= 0.0 -> no button (shopUI+0x2e8 untouched)
...
141fb0024  MOVSD  XMM0,[R15+R14+0x40]
141fb002b  COMISD XMM0,XMM6
141fb002f  JBE    0x141fb00cf             ; <= 0.0 -> the ordinary price label
141fb0050  CALL   0x142f23580             ; ceil
141fb0183  MOV    EDX,0x4b0               ; 'Recharge: %lld'
```

With `+0x10c = 1` the price is `(1 - count) x unit`: **zero or below for every stack holding one
star or more**. The button was never drawn, so no `0x00F5` sub-op 2 was ever sent - which is what
every fixture shows. **[D]** One stack *was* priced above zero all along: an **empty** one,
`(1 - 0) x unit`. Nobody is recorded trying that.

Cross-checks: the decompiler (`(double)(*(int*)(row+0x10c) - *(int*)(entry+0xc)) * *(double*)(row+0x40)`)
and the listing agree on operands and order. The row decoder stores the field with
`movsx ecx, ax` at `1404ba5c2` - a signed `i16` widened to the dword read here, so slotMax 500 or
800 arrives intact.

The Recharge builder `FUN_141fb9240` refuses only `== 0.0`, so a *negative* price would have been
sent if the button could be pressed. The button is the gate, not the builder.

## 2. The lead in `STATUS.md` was wrong, and how

It guessed the builder looked the price up in `+0x340` or a Buy tab, which a price-0 row never
reaches. It looks in **`+0x358`, the Recharge list** (`141fa7a00`), and the row loop puts every
`207`/`233` row there, price 0 included, at `141fa001a`, before the destroy at `141fa030c`. So
the recharge-only rows *were* found. They just priced every stack at `(1 - count) x unit`, the same
as a stocked Subi row. The guess was marked [I] and nothing was built on it.

## 3. Why the Buy row cannot simply carry the full stack

On a Buy row, `+0x10c` is the per-purchase cap (`classic-shop-rows.md` §3 row 42): `== 1` is a
yes/no, `> 1` a quantity box with that ceiling. A star is sold **one full set per purchase** (the
owner, 2026-10-02 and 2026-10-03), so its Buy row must stay at 1. The same field cannot be both.

The way out is the first-match lookup. Every star a counter sells now goes out as **two rows**: a
price-0 recharge-only row with the full stack in `+0x10c`, **immediately before** the Buy row.
The recharge-only row is first in `+0x358`, so it prices the recharge. The client destroys it before
any Buy tab, so the Buy row and its yes/no are unchanged.

## 4. The Buy index skips recharge-only rows. **[L]**

A Buy request's `rowIndex` comes from `shopUI+0x390`, written at `141fa0b40` as
`[rbp+4] - 1 + len(shopUI+0x340)`, right after the row joins `+0x340`. `[rbp+4]` is written only at
`141f9f851`, `141f9fa52`, `141fa035c`, `141fa0576` and `141fa0748`: the rows the loop passes over
but counts. The price-0 path `141fa030c -> 141fa1178 -> 141fa1219 -> 141fa1224` reloads `[rbp+4]`
and never increments it. **So a recharge-only row takes no index, and every Buy row behind one
is sent with an index one lower than its position in the packet.**

That is why the general-store recharge-only rows were appended at the end. With rows now ahead of
a stocked star, the server maps the index (`net::classicshop::buy_row_index`): the n-th row that
is not recharge-only. `classic_buy` still checks the item id the client names, so a wrong mapping is
refused and logged as *"row N is item X and the client asked for Y"*. It never sells the wrong
thing.

**Where this is observable:** only where a row follows a stocked star. In `data/shops.txt` that is
one counter, **Max** (Kerning City Civic Center): Unagi, Grape Juice and Elixir come after Wolbi.
At every Grocer, Subi is the last row.

## 5. The label rounds up, like the server. **[L]**

`141fb0049..141fb0055`: `ceil(entry+0x40)`, then (when `FUN_141fb9f30` returns 0) the number
formatted with string `0x4B0`. `classic_recharge` charges `ceil(missing x unit)`. These agree,
**unless** `FUN_141fb9f30` returns a discounted price. Its inputs (`FUN_142ce51a0`,
`FUN_141fba230`, `FUN_1402c9510`) are not traced, and the last one sits beside the citizenship
checks `FUN_1402c90f0` / `FUN_1402c9150`. A label lower than the charge would be that discount.

## 6. Still unmeasured

* **[I]** That the button now appears. Every link is [L]; none has been seen on a screen.
* **[I]** The other conditions on a Sell entry (`FUN_14038a860`, `FUN_14038bf90`,
  `FUN_140389c10`, `FUN_1403e1180` at `141fa78d1`..`141fa7959`) decide whether a star is in the
  Sell list at all. The owner has sold stars, so they pass, but that was not checked here.
* **[I]** A stack larger than slotMax (claw mastery) prices below zero and gets no button. The
  server refuses it too (`have >= slot_max`). Both are consistent, and neither has been seen.

## 7. The fix (2026-10-03)

* `ClassicShopRow::recharge_only(id, unit, slot_max)` puts slotMax in `+0x10c`.
* `open_shop_for` puts one in front of every stocked star (any counter). A general store also
  appends one for every star it does not sell.
* `classic_buy` maps the client's index past them with `buy_row_index`.
* Tests:
  * `a_recharge_only_row_makes_the_window_price_every_partial_stack_above_zero` models §1 on the
    bytes off the wire. The old `+0x10c = 1` gives no button for 1, 2 or 499 stars; the new row
    gives `(500 - n) x 0.4`.
  * `a_grocers_subi_is_priced_for_recharge_from_a_full_stack_row_ahead_of_its_buy_row`.
  * `a_buy_index_skips_recharge_only_rows`.
